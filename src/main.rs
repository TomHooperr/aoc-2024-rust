use std::{env, fs};
use std::time::Instant;

mod day01;

fn elapsed_since(start_time: &Instant) -> String {
    let elapsed = start_time.elapsed().as_micros();
    if elapsed >= 1_000_000 {
        let elapsed = elapsed as f64 / 1_000_000.0;
        format!("{elapsed:.1}s")
    } else if elapsed >= 1000 {
        let elapsed = elapsed as f64 / 1000.0;
        format!("{elapsed:.1}ms")
    } else {
        format!("{elapsed}µs")
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // Remove "--profile" and "dev" args
    if args.ends_with(&["--profile".to_string(), "dev".to_string()]) {
        args.truncate(args.len() - 2);
    }
    let days: Vec<_> = match &args.len() {
        // If no days specified, run all.
        1 => (1..=25).collect(),
        // else run specified days
        _ => args.iter().skip(1).map(|d| d.parse().unwrap()).collect()
    };

    let global_start_time = Instant::now();

    for day in &days {
        println!("Day {}:", day);

        let path = format!("./data/day{:02}.txt", day);
        let input = fs::read_to_string(&path);
        let start_time = Instant::now();

        // If input data exists...
        if let Ok(input) = input {
            let input = input.trim_end();

            // Find the associated solution function
            let day_func = match day {
                1 => day01::run,
                _ => unreachable!()
            };

            // Run solution
            day_func(input);
            println!("Time: {}", elapsed_since(&start_time));
        } else {
            println!("No data!");
        }
        println!();
    }
    if days.len() > 1 {
        println!("Total Time: {}", elapsed_since(&global_start_time));
    }
}
