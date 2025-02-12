// Copyright 2025 Felipe Torres González
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use clap::{CommandFactory, Parser};
use data_harvest::{feeders::IbexShortFeeder, web_scrappers::CnmvProvider};
use finance_data_harvester::telemetry;
use secrecy::{ExposeSecret, SecretString};
use sqlx::postgres::PgPool;
use std::env;
use std::sync::Arc;
use tracing::{error, info, warn};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[arg(
        long,
        value_name = "ticker list | all",
        help = "Refresh short positions."
    )]
    refresh_short: Option<Vec<String>>,

    #[arg(long, help = "Run as a systemd service")]
    service: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // First: detect wether the binary runs as a systemd service, so the
    // tracing subsystem can enable logging to journald.
    let running_under_systemd = if cli.service {
        true
    } else {
        match env::var("RUNNING_AS_SERVICE") {
            Ok(val) => matches!(val.as_str(), "y" | "Y" | "yes" | "YES"),
            Err(_) => false,
        }
    };

    telemetry::configure_tracing(running_under_systemd, cli.debug);

    if running_under_systemd {
        info!("Finance Data Harvester - running as a systemd service");
    } else {
        info!("Finance Data Harvester");
    }

    let database_url = match env::var("DATABASE_URL") {
        Ok(val) => SecretString::from(val),
        Err(_) => {
            error!("Missing environment variable DATABASE_URL");
            return;
        }
    };

    let db_pool = match PgPool::connect(database_url.expose_secret()).await {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to connect to the DB: {e}");
            return;
        }
    };

    let feeder = IbexShortFeeder {
        scrapper: Arc::new(CnmvProvider::new()),
        pool: &db_pool,
    };

    match &cli.refresh_short {
        Some(ticker) => {
            if ticker.len() > 1 {
                warn!("Feature not supported");
            } else if ticker[0] != "all" {
                warn!("Feature not supported");
            } else {
                info!(
                    "Selected to refresh short positions for the tickers: {:?}",
                    ticker
                );
                let _ = feeder.add_today_data().await.map_err(|e| {
                    error!("{e}");
                });
                info!("Short positions updated");
            }
        }
        None => Cli::command().print_help().unwrap(),
    }

    info!("Harvesting finished!");
}
