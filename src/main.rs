use aoc_2021::*;
use clap::StructOpt;
use cli::Command::*;

pub mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match cli::Arguments::parse().command {
        DAY1 { path } => cli::aoc_2021_1_cli::execute(path),
        DAY1B { path } => cli::aoc_2021_1b_cli::execute(path),
        DAY2 { path } => cli::aoc_2021_2_cli::execute(path),
        DAY2B { path } => cli::aoc_2021_2b_cli::execute(path),
        DAY3 { path } => cli::aoc_2021_3_cli::execute(path),
        DAY3B { path } => cli::aoc_2021_3b_cli::execute(path),
        DAY4 { path } => cli::aoc_2021_4_cli::execute(path),
        DAY4B { path } => cli::aoc_2021_4b_cli::execute(path),
        DAY5 { path } => cli::aoc_2021_5_cli::execute(path),
        DAY5B { path } => cli::aoc_2021_5b_cli::execute(path),
        DAY6 { path, life_cycles } => cli::aoc_2021_6_cli::execute(path, life_cycles),
        DAY6B { path, life_cycles } => cli::aoc_2021_6b_cli::execute(path, life_cycles),
        DAY7 { path } => cli::aoc_2021_7_cli::execute(path),
        DAY7B { path } => cli::aoc_2021_7b_cli::execute(path),
        DAY9 { path } => cli::aoc_2021_9_cli::execute(path),
        DAY9B { path } => cli::aoc_2021_9b_cli::execute(path),
        DAY10 { path } => cli::aoc_2021_10_cli::execute(path),
        DAY10B { path } => cli::aoc_2021_10b_cli::execute(path),
        DAY11 { path } => cli::aoc_2021_11_cli::execute(path),
        DAY11B { path } => cli::aoc_2021_11b_cli::execute(path),
        DAY12 { path } => cli::aoc_2021_12_cli::execute(path),
        DAY12B { path } => cli::aoc_2021_12b_cli::execute(path),
        DAY13 { path } => cli::aoc_2021_13_cli::execute(path),
        DAY13B { path } => cli::aoc_2021_13b_cli::execute(path),
        DAY14 { path } => cli::aoc_2021_14_cli::execute(path),
        DAY14B { path } => cli::aoc_2021_14b_cli::execute(path),
        DAY15 { path } => cli::aoc_2021_15_cli::execute(path),
        DAY15B { path } => cli::aoc_2021_15b_cli::execute(path),
        DAY16 { path } => cli::aoc_2021_16_cli::execute(path),
        DAY16B { path } => cli::aoc_2021_16b_cli::execute(path),
    }
}
