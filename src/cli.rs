use std::path::PathBuf;

use clap::{AppSettings, Parser, Subcommand};

pub mod aoc_2021_1_cli;
pub mod aoc_2021_1b_cli;
pub mod aoc_2021_2_cli;
pub mod aoc_2021_2b_cli;
pub mod aoc_2021_3_cli;
pub mod aoc_2021_3b_cli;
pub mod aoc_2021_4_cli;
pub mod aoc_2021_4b_cli;
pub mod aoc_2021_5_cli;
pub mod aoc_2021_5b_cli;
pub mod aoc_2021_6_cli;
pub mod aoc_2021_6b_cli;
pub mod aoc_2021_7_cli;
pub mod aoc_2021_7b_cli;
//pub mod aoc_2021_8_cli;
//pub mod aoc_2021_8b_cli;
pub mod aoc_2021_9_cli;
pub mod aoc_2021_9b_cli;

pub mod aoc_2021_10_cli;
pub mod aoc_2021_10b_cli;
pub mod aoc_2021_11_cli;
pub mod aoc_2021_11b_cli;
pub mod aoc_2021_12_cli;
pub mod aoc_2021_12b_cli;
pub mod aoc_2021_13_cli;
pub mod aoc_2021_13b_cli;
pub mod aoc_2021_14_cli;
pub mod aoc_2021_14b_cli;

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
    #[clap(name = "day-3", setting(AppSettings::ArgRequiredElseHelp))]
    DAY3 {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-3b", setting(AppSettings::ArgRequiredElseHelp))]
    DAY3B {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-4", setting(AppSettings::ArgRequiredElseHelp))]
    DAY4 {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-4b", setting(AppSettings::ArgRequiredElseHelp))]
    DAY4B {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-5", setting(AppSettings::ArgRequiredElseHelp))]
    DAY5 {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-5b", setting(AppSettings::ArgRequiredElseHelp))]
    DAY5B {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-6", setting(AppSettings::ArgRequiredElseHelp))]
    DAY6 {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
        #[clap(required = true)]
        life_cycles: u32,
    },
    #[clap(name = "day-6b", setting(AppSettings::ArgRequiredElseHelp))]
    DAY6B {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
        #[clap(required = true)]
        life_cycles: u32,
    },
    #[clap(name = "day-7", setting(AppSettings::ArgRequiredElseHelp))]
    DAY7 {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-7b", setting(AppSettings::ArgRequiredElseHelp))]
    DAY7B {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-9", setting(AppSettings::ArgRequiredElseHelp))]
    DAY9 {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-9b", setting(AppSettings::ArgRequiredElseHelp))]
    DAY9B {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-10", setting(AppSettings::ArgRequiredElseHelp))]
    DAY10 {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-10b", setting(AppSettings::ArgRequiredElseHelp))]
    DAY10B {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-11", setting(AppSettings::ArgRequiredElseHelp))]
    DAY11 {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-11b", setting(AppSettings::ArgRequiredElseHelp))]
    DAY11B {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-12", setting(AppSettings::ArgRequiredElseHelp))]
    DAY12 {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-12b", setting(AppSettings::ArgRequiredElseHelp))]
    DAY12B {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-13", setting(AppSettings::ArgRequiredElseHelp))]
    DAY13 {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-13b", setting(AppSettings::ArgRequiredElseHelp))]
    DAY13B {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-14", setting(AppSettings::ArgRequiredElseHelp))]
    DAY14 {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
    #[clap(name = "day-14b", setting(AppSettings::ArgRequiredElseHelp))]
    DAY14B {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
}
