#![feature(array_windows)]
#![feature(iter_intersperse)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use clap::{Parser, Subcommand};
use day1::{day1_1, day1_2};
use day2::{day2_1, day2_2};
use day3::{day3_1, day3_2};
use day4::{day4_1, day4_2};
use day5::{day5_1, day5_2};
use day6::{day6_1, day6_2};
use day7::{day7_1, day7_2};
use day8::{day8_1, day8_2};
use day9::{day9_1, day9_2};

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
    Day5_1,
    Day5_2,
    Day6_1,
    Day6_2,
    Day7_1,
    Day7_2,
    Day8_1,
    Day8_2,
    Day9_1,
    Day9_2,
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
        Commands::Day5_1 => day5_1(),
        Commands::Day5_2 => day5_2(),
        Commands::Day6_1 => day6_1(),
        Commands::Day6_2 => day6_2(),
        Commands::Day7_1 => day7_1(),
        Commands::Day7_2 => day7_2(),
        Commands::Day8_1 => day8_1(),
        Commands::Day8_2 => day8_2(),
        Commands::Day9_1 => day9_1(),
        Commands::Day9_2 => day9_2(),
    }
}
