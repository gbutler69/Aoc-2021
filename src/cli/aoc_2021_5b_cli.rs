use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use itertools::Itertools;

use aoc_2021::aoc_2021_5::Vector2D;
use aoc_2021::aoc_2021_5b::determine_number_of_points_with_overlapping;

use crate::cli::aoc_2021_5_cli::split_to_owned;

pub fn execute(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let number_of_overlapping_points = determine_number_of_points_with_overlapping(
        io::BufReader::new(File::open(path)?)
            .lines()
            .filter_ok(|line| !line.trim().is_empty())
            .map_ok(|line| split_to_owned(line, " -> "))
            .map_ok(|(p1, p2)| (split_to_owned(p1, ","), split_to_owned(p2, ",")))
            .map_ok(|((x1, y1), (x2, y2))| {
                (
                    (x1.parse::<u32>().unwrap(), y1.parse::<u32>().unwrap()),
                    (x2.parse::<u32>().unwrap(), y2.parse::<u32>().unwrap()),
                )
            })
            .map_ok(Vector2D::from)
            .collect::<Result<Vec<_>, _>>()?,
    );
    println!("overlapping points {number_of_overlapping_points}");
    Ok(())
}
