use std::path::PathBuf;

use clap::{AppSettings, Parser, Subcommand};

pub mod aoc_2021_1_cli;
pub mod aoc_2021_1b_cli;
pub mod aoc_2021_2_cli;
pub mod aoc_2021_2b_cli;

/// About Me!
///
/// What do I want to say in the long about?
#[derive(Parser, Debug)]
pub struct Arguments {
    /// About command!
    ///
    /// Long About command!
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Run the Day 1 Challenge
    #[clap(name = "day-1", setting(AppSettings::ArgRequiredElseHelp))]
    DAY1 {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-1b", setting(AppSettings::ArgRequiredElseHelp))]
    DAY1B {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-2", setting(AppSettings::ArgRequiredElseHelp))]
    DAY2 {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-2b", setting(AppSettings::ArgRequiredElseHelp))]
    DAY2B {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
}
