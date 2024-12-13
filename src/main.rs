#![feature(array_windows)]
#![feature(iter_intersperse)]
#![feature(iter_chain)]
extern crate core;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day15_2018;
mod day11;
mod day12;
mod day13;

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
use crate::day10::{day10_1, day10_2};
use crate::day11::{day11_1, day11_2};
use crate::day15_2018::day15_1_2018;
use crate::day12::{day12_1, day12_2};
use crate::day13::{day13_1, day13_2};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Day15_1_2018,
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
    Day10_1,
    Day10_2,
    Day11_1,
    Day11_2,
    Day12_1,
    Day12_2,
    Day13_1,
    Day13_2,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Day15_1_2018 => day15_1_2018(),
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
        Commands::Day10_1 => day10_1(),
        Commands::Day10_2 => day10_2(),
        Commands::Day11_1 => day11_1(),
        Commands::Day11_2 => day11_2(),
        Commands::Day12_1 => day12_1(),
        Commands::Day12_2 => day12_2(),
        Commands::Day13_1 => day13_1(),
        Commands::Day13_2 => day13_2(),
    }
}
