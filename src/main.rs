use std::{fs::File, sync::Arc};

use chrono::{Duration, Local, NaiveDate};
use clap::Parser;
use tracing_subscriber::{filter, prelude::*, EnvFilter};

use aoc_2022::*;

#[derive(Parser)]
pub struct Cli {
    /// Day to run
    #[arg(short, long, env = "DAY")]
    day: Option<usize>,

    /// Use Example flag
    #[arg(short, long)]
    example: bool,
}

fn init_logging() {
    let stdout_log = tracing_subscriber::fmt::layer().pretty();

    // A layer that logs events to a file.
    let file = File::create("debug.log");
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error: {error:?}"),
    };
    let debug_log = tracing_subscriber::fmt::layer().with_writer(Arc::new(file));

    // A layer that collects metrics using specific events.
    let metrics_layer = /* ... */ filter::LevelFilter::INFO;

    tracing_subscriber::registry()
        .with(
            stdout_log
                // Add an `INFO` filter to the stdout logging layer
                .with_filter(filter::LevelFilter::INFO)
                // Combine the filtered `stdout_log` layer with the
                // `debug_log` layer, producing a new `Layered` layer.
                .and_then(debug_log)
                // Add a filter to *both* layers that rejects spans and
                // events whose targets start with `metrics`.
                .with_filter(filter::filter_fn(|metadata| !metadata.target().starts_with("metrics"))),
        )
        .with(
            // Add a filter to the metrics label that *only* enables
            // events whose targets start with `metrics`.
            metrics_layer.with_filter(filter::filter_fn(|metadata| metadata.target().starts_with("metrics"))),
        )
        .with(EnvFilter::from_default_env())
        .init();
}

fn main() {
    let args = Cli::parse();
    init_logging();
    let day = {
        if let Some(day) = args.day {
            day as i64
        } else {
            let start = NaiveDate::from_ymd_opt(2022, 12, 1)
                .unwrap()
                .and_hms_milli_opt(0, 0, 0, 0)
                .unwrap()
                .and_local_timezone(Local)
                .unwrap();
            let today = Local::now();
            let diff = today - start + Duration::days(1);
            diff.num_days()
        }
    };

    match day {
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(),
        4 => day04::run(),
        5 => day05::run(),
        6 => day06::run(),
        7 => day07::run(args.example),
        8 => day08::run(args.example),
        _ => {
            eprintln!("Day not found: `{day}`. Running last available day (day=8)");
            day08::run(args.example);
        }
    }
}
