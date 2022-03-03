use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use itertools::Itertools;

use aoc_2021::aoc_2021_13::{determine_number_of_points_after_folds, Axis};

pub fn execute(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut points = Vec::new();
    let mut folds = Vec::new();
    for line in io::BufReader::new(File::open(path)?)
        .lines()
        .filter_ok(ignore_empty_or_all_whitespace_lines)
    {
        let line = line?;
        match line.starts_with("fold along") {
            true => folds.push(split_to_fold_tuple(line)),
            false => points.push(split_to_point_tuple(line)),
        }
    }
    let total_points = determine_number_of_points_after_folds(points, folds.into_iter().take(1));
    println!("total points {total_points}");
    Ok(())
}

#[allow(clippy::ptr_arg)]
fn ignore_empty_or_all_whitespace_lines(line: &String) -> bool {
    !line.trim().is_empty()
}

fn split_to_fold_tuple(line: String) -> (Axis, u32) {
    let fold = line
        .split_once("fold along ")
        .unwrap()
        .1
        .split_once("=")
        .unwrap();
    (Axis::from(fold.0), fold.1.parse::<u32>().unwrap())
}

fn split_to_point_tuple(line: String) -> (u32, u32) {
    line.split_once(",")
        .map(|point| {
            (
                point.0.parse::<u32>().unwrap(),
                point.1.parse::<u32>().unwrap(),
            )
        })
        .unwrap()
}
