use aoc_2021::*;
use clap::StructOpt;

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match cli::Arguments::parse().command {
        cli::Command::DAY1 { path } => cli::aoc_2021_1_cli::execute(path),
        cli::Command::DAY1B { path } => cli::aoc_2021_1b_cli::execute(path),
        cli::Command::DAY2 { path } => cli::aoc_2021_2_cli::execute(path),
        cli::Command::DAY2B { path } => cli::aoc_2021_2b_cli::execute(path),
        cli::Command::DAY3 { path } => cli::aoc_2021_3_cli::execute(path),
        cli::Command::DAY3B { path } => cli::aoc_2021_3b_cli::execute(path),
    }
}
