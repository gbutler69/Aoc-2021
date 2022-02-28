use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use itertools::Itertools;

use aoc_2021::aoc_2021_11b::determine_step_where_all_octopi_flash;

pub fn execute(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let lines = io::BufReader::new(File::open(path)?)
        .lines()
        .filter_ok(|line| !line.trim().is_empty())
        .map_ok(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let step_where_all_flash = determine_step_where_all_octopi_flash(lines, 60_000).unwrap();
    println!("step where all flash {step_where_all_flash}");
    Ok(())
}
