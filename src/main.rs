mod day1;
mod macros;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Optional name to operate on
    day: u8,
    part: u8,
    data: String,
}

fn get_content(day: u8, data: String) -> String {
    let filename = format!("day{}.txt", day);
    let mut file = File::open(PathBuf::from(data).join(Path::new(&filename))).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s
}

make_days_map!(run_day, {
    1 => day1,
    2 => day2,
    3 => day3,
    4 => day4,
    5 => day5,
    6 => day6,
    7 => day7,
    8 => day8,
    9 => day9,
    10 => day10
});

fn main() {
    let args = Args::parse();
    run_day(args.day, args.part, get_content(args.day, args.data));
}
