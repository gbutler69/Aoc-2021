use aoc_2021::*;
use clap::StructOpt;
use cli::Command::*;

mod cli;

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
    }
}
