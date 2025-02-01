use clap::Parser;
use std::fs;

mod days;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    day: u8,

    #[arg(short, long, default_value = "false")]
    part_two: bool,

    #[arg(short, long, default_value = "false")]
    test: bool,
}

fn main() {
    let args = Args::parse();

    let input_file = format!(
        "inputs/day{}{}.txt",
        args.day,
        if args.test { "short" } else { "" }
    );

    let content = fs::read_to_string(&input_file).unwrap_or_else(|e| {
        panic!(
            "Failed to read input for day {} (path: \"{}\") with error: {}",
            args.day, &input_file, e
        )
    });

    let output = match args.day {
        1 => days::day1::solve(&content, args.part_two),
        _ => format!("Day {} not solved yet", args.day),
    };

    println!(
        "Solution for day {} part {}:",
        args.day,
        if args.part_two { "two" } else { "one" }
    );
    println!("{}", output);
}
