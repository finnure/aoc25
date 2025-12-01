extern crate day01;
extern crate day02;
extern crate day03;
extern crate day04;
extern crate day05;
extern crate day06;
extern crate day07;
extern crate day08;
extern crate day09;
extern crate day10;
extern crate day11;
extern crate day12;
extern crate utils;
use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    run(config).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        process::exit(1);
    });
}

struct Config {
    day: u32,
    debug: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip executable name

        let day = match args.next() {
            Some(arg) => arg.parse::<u32>().map_err(|_| "Day must be a number")?,
            None => return Err("Didn't get a day"),
        };

        let debug = match args.next() {
            Some(arg) => arg == "debug",
            None => false,
        };

        Ok(Config { day, debug })
    }
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let input = utils::read_input(config.day, config.debug)?;

    match config.day {
        1 => day01::solve(&input),
        2 => day02::solve(&input),
        3 => day03::solve(&input),
        4 => day04::solve(&input),
        5 => day05::solve(&input),
        6 => day06::solve(&input),
        7 => day07::solve(&input),
        8 => day08::solve(&input),
        9 => day09::solve(&input),
        10 => day10::solve(&input),
        11 => day11::solve(&input),
        12 => day12::solve(&input),

        _ => {
            println!("Day {} is not implemented yet.", config.day);
        }
    }

    Ok(())
}
