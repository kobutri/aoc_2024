#![feature(array_windows)]
#![feature(iter_intersperse)]

mod day1;
mod day2;
mod day3;
mod day4;
use clap::{Parser, Subcommand};
use day1::{day1_1, day1_2};
use day2::{day2_1, day2_2};
use day3::{day3_1, day3_2};
use day4::{day4_1, day4_2};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Day1_1,
    Day1_2,
    Day2_1,
    Day2_2,
    Day3_1,
    Day3_2,
    Day4_1,
    Day4_2,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Day1_1 => day1_1(),
        Commands::Day1_2 => day1_2(),
        Commands::Day2_1 => day2_1(),
        Commands::Day2_2 => day2_2(),
        Commands::Day3_1 => day3_1(),
        Commands::Day3_2 => day3_2(),
        Commands::Day4_1 => day4_1(),
        Commands::Day4_2 => day4_2(),
    }
}
