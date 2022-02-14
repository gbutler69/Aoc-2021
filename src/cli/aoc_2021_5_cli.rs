use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use itertools::Itertools;

use aoc_2021::aoc_2021_5::{determine_number_of_points_with_overlapping, Vector2D};

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

pub fn split_to_owned(line: String, delimiter: &str) -> (String, String) {
    let (a, b) = line.split_once(delimiter).unwrap();
    (a.to_owned(), b.to_owned())
}
