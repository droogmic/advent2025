use advent2025_lib::{DayTrait, Part, PrimaryExample, get_days};
use clap::{Arg, Command};
use color_eyre::Report;
use colored::*;

fn build_cli() -> Command {
    Command::new("advent2025")
        .arg(
            Arg::new("puzzle")
                .value_parser(clap::value_parser!(usize))
                .required(false),
        )
        .arg(Arg::new("all").long("all").action(clap::ArgAction::SetTrue))
        .arg(
            Arg::new("parallel")
                .long("parallel")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("example")
                .long("example")
                .action(clap::ArgAction::SetTrue),
        )
}

fn print_day<O: std::fmt::Display>(
    day_num: usize,
    display: (&'static str, &'static str),
    result: (O, O),
) {
    println!("Day {}", day_num);
    println!(
        "Part 1: {}",
        display.0.replace("{answer}", &result.0.to_string())
    );
    println!(
        "Part 2: {}",
        display.1.replace("{answer}", &result.1.to_string())
    );
    println!();
}

fn main() -> Result<(), Report> {
    setup()?;

    println!("{}", "Advent Of Code 2022".bold().blue());
    println!();

    let matches = build_cli().get_matches();
    let puzzle = matches.get_one::<usize>("puzzle").copied();
    let all = matches.get_flag("all");
    let parallel = matches.get_flag("parallel");
    let example = matches.get_flag("example");
    let days = get_days();

    let get_result_pair = move |day: &dyn DayTrait| -> (String, String) {
        if example {
            match day.examples() {
                PrimaryExample::Same(example) => {
                    day.both(example).expect("example should be valid")
                }
                PrimaryExample::Different([first, second]) => (
                    day.calc(Part::First, first).unwrap(),
                    day.calc(Part::Second, second).unwrap(),
                ),
            }
        } else {
            day.both(&day.input()).expect("invalid input")
        }
    };

    if all {
        for (day_num, day) in days.into_iter() {
            print_day(day_num, day.display(), get_result_pair(day.as_ref()));
        }
    } else if parallel {
        let threads = get_days().into_iter().map(|(day_num, day)| {
            println!("Spawn day {}", day_num);
            std::thread::spawn(move || (day_num, day.display(), get_result_pair(day.as_ref())))
        });
        std::thread::yield_now();
        std::thread::sleep(std::time::Duration::from_millis(50));
        println!();
        for thread in threads {
            let (day_num, display, (part1, part2)) = thread.join().unwrap();
            print_day(day_num, display, (part1, part2));
        }
    } else if !(all || parallel) {
        let (day_num, day): (usize, _) = match puzzle {
            None => {
                let (last_day_num, last_day) = days.iter().next_back().unwrap();
                (*last_day_num, last_day)
            }
            Some(day_num) => (day_num, days.get(&day_num).unwrap()),
        };
        print_day(day_num, day.display(), get_result_pair(day.as_ref()));
    }

    Ok(())
}

fn setup() -> Result<(), Report> {
    // if std::env::var("RUST_BACKTRACE").is_err() {
    //     std::env::set_var("RUST_BACKTRACE", "1")
    // }
    color_eyre::install()?;

    env_logger::init();
    log::info!("Starting Logging");

    Ok(())
}
