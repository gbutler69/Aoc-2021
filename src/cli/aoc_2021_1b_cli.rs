use std::{
    fmt::Display,
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use itertools::Itertools;

pub fn execute(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}",
        crate::aoc_2021_1b::count_increased_measurements(
            io::BufReader::new(File::open(path)?)
                .lines()
                .map_ok(|line| line.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?
        )
    );
    Ok(())
}

#[derive(Debug)]
enum MyError {
    FileNotFound(String),
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::FileNotFound(path) => path.fmt(f),
        }
    }
}

impl std::error::Error for MyError {}
