mod day1;
mod macros;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Optional name to operate on
    day: u8,
    part: u8,
}

fn get_content(day: u8, part: u8) -> String {
    format!("({day}, {part})")
}

make_days_map!(run_day, get_content, {
    1 => day1
});

fn main() {
    let args = Args::parse();
    run_day(args.day, args.part);
}
