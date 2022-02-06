use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use itertools::Itertools;

pub fn execute(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}",
        crate::aoc_2021_1::count_increased_measurements(
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
