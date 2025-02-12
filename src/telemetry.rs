// Copyright 2025 Felipe Torres González
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use tracing::{error, level_filters::LevelFilter};
use tracing_subscriber::{fmt, prelude::*, Layer};

pub fn configure_tracing(enable_journald: bool, verbose_level: u8) {
    // Store all the tracing layers in an array to allow a dynamic configuration
    // using the given settings to the app.
    let mut layers = Vec::new();

    // Map the verbosity level to a LevelFilter
    let tracing_level = match verbose_level {
        0 => LevelFilter::ERROR,
        1 => LevelFilter::INFO,
        2 => LevelFilter::WARN,
        3 => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE,
    };

    // Configure the default layer: STDOUT
    let layer = fmt::layer()
        .with_ansi(false)
        .with_target(false)
        .with_filter(tracing_level)
        .boxed();

    layers.push(layer);

    if enable_journald {
        match tracing_journald::layer() {
            Ok(layer) => {
                layers.push(
                    layer
                        .with_field_prefix(Some("finance_harvester".to_owned()))
                        .boxed(),
                );
            }
            // journald is typically available on Linux systems, but nowhere else. Portable software
            // should handle its absence gracefully.
            Err(e) => {
                error!("couldn't connect to journald: {e}");
            }
        }
    }

    tracing_subscriber::registry().with(layers).init();
}
