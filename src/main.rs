use clap::{Parser, Subcommand};
use crate::day15_2018::day15_1_2018;
use crate::day1::{day1_1, day1_2};
use crate::day10::{day10_1, day10_2};
use crate::day11::{day11_1, day11_2};
use crate::day12::{day12_1, day12_2};
use crate::day13::{day13_1, day13_2};
use crate::day14::{day14_1, day14_2};
use crate::day15::{day15_1, day15_2};
use crate::day16::{day16_1, day16_2};
use crate::day17::{day17_1, day17_2};
use crate::day18::{day18_1, day18_2};
use crate::day19::{day19_1, day19_2};
use crate::day2::{day2_1, day2_2};
use crate::day20::{day20_1, day20_2};
use crate::day21::{day21_1, day21_2};
use crate::day22::{day22_1, day22_2};
use crate::day23::{day23_1, day23_2};
use crate::day24::{day24_1, day24_2};
use crate::day3::{day3_1, day3_2};
use crate::day4::{day4_1, day4_2};
use crate::day5::{day5_1, day5_2};
use crate::day6::{day6_1, day6_2};
use crate::day7::{day7_1, day7_2};
use crate::day8::{day8_1, day8_2};
use crate::day9::{day9_1, day9_2};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day15_2018;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;



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
    Day14_1,
    Day14_2,
    Day15_1,
    Day15_2,
    Day16_1,
    Day16_2,
    Day17_1,
    Day17_2,
    Day18_1,
    Day18_2,
    Day19_1,
    Day19_2,
    Day20_1,
    Day20_2,
    Day21_1,
    Day21_2,
    Day22_1,
    Day22_2,
    Day23_1,
    Day23_2,
    Day24_1,
    Day24_2,
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
        Commands::Day14_1 => day14_1(),
        Commands::Day14_2 => day14_2(),
        Commands::Day15_1 => day15_1(),
        Commands::Day15_2 => day15_2(),
        Commands::Day16_1 => day16_1(),
        Commands::Day16_2 => day16_2(),
        Commands::Day17_1 => day17_1(),
        Commands::Day17_2 => day17_2(),
        Commands::Day18_1 => day18_1(),
        Commands::Day18_2 => day18_2(),
        Commands::Day19_1 => day19_1(),
        Commands::Day19_2 => day19_2(),
        Commands::Day20_1 => day20_1(),
        Commands::Day20_2 => day20_2(),
        Commands::Day21_1 => day21_1(),
        Commands::Day21_2 => day21_2(),
        Commands::Day22_1 => day22_1(),
        Commands::Day22_2 => day22_2(),
        Commands::Day23_1 => day23_1(),
        Commands::Day23_2 => day23_2(),
        Commands::Day24_1 => day24_1(),
        Commands::Day24_2 => day24_2(),
    }
}
