// Copyright 2025 Felipe Torres González
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use tracing::{error, level_filters::LevelFilter, Level};
use tracing_subscriber::{filter::Targets, fmt, prelude::*, Layer};

pub fn configure_tracing(enable_journald: bool, verbose_level: u8) {
    // Store all the tracing layers in an array to allow a dynamic configuration
    // using the given settings to the app.
    let mut layers = Vec::new();

    // Map the verbosity level to a LevelFilter
    let (tracing_level, tracing_levelfilter) = match verbose_level {
        0 => (Level::ERROR, LevelFilter::ERROR),
        1 => (Level::INFO, LevelFilter::INFO),
        2 => (Level::WARN, LevelFilter::WARN),
        3 => (Level::DEBUG, LevelFilter::DEBUG),
        _ => (Level::TRACE, LevelFilter::TRACE),
    };

    if enable_journald {
        match tracing_journald::layer() {
            Ok(layer) => {
                layers.push(
                    layer
                        .with_field_prefix(Some("finance_harvester".to_owned()))
                        .with_filter(tracing_levelfilter)
                        .boxed(),
                );
            }
            // journald is typically available on Linux systems, but nowhere else. Portable software
            // should handle its absence gracefully.
            Err(e) => {
                error!("couldn't connect to journald: {e}");
            }
        }
    } else {
        // Configure the default layer: STDOUT when not running as a systemd service
        let layer = fmt::layer()
            .with_ansi(false)
            .with_target(false)
            .with_filter(tracing_levelfilter)
            .boxed();

        layers.push(layer);
    }

    // Enable a target to avoid receiving logs from hyper and the scrapper crates (too verbose).
    let target = Targets::new()
        .with_target("finance_data_harvester", tracing_level)
        .with_target("data_harvest", tracing_level);

    tracing_subscriber::registry()
        .with(layers)
        .with(target)
        .init();
}
