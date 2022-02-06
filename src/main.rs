use aoc_2021::*;
use clap::StructOpt;

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match cli::Arguments::parse().command {
        cli::Command::DAY1 { path } => cli::aoc_2021_1_cli::execute(path),
        cli::Command::DAY1B { path } => cli::aoc_2021_1b_cli::execute(path),
    }
}
