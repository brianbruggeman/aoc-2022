use chrono::{Duration, Local, NaiveDate};
use clap::Parser;

use aoc_2022::*;

#[derive(Parser)]
pub struct Cli {
    /// Day to run
    day: Option<usize>,
}

fn main() {
    let args = Cli::parse();
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
        _ => {
            eprintln!(
                "Day not found: `{}`. Running last available day (day=3)",
                day
            );
            day03::run();
        }
    }
}
