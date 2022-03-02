use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use itertools::Itertools;

use aoc_2021::aoc_2021_12b::determine_number_of_paths_from_start_to_end;

pub fn execute(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let edges = io::BufReader::new(File::open(path)?)
        .lines()
        .filter_ok(ignore_empty_or_all_whitespace_lines)
        .map_ok(split_to_edge_tuple)
        .collect::<Result<Vec<_>, _>>()?;
    let total_paths = determine_number_of_paths_from_start_to_end(edges);
    println!("total paths {total_paths}");
    Ok(())
}

#[allow(clippy::ptr_arg)]
fn ignore_empty_or_all_whitespace_lines(line: &String) -> bool {
    !line.trim().is_empty()
}

fn split_to_edge_tuple(line: String) -> (String, String) {
    line.split_once("-")
        .map(|edge| (edge.0.into(), edge.1.into()))
        .unwrap()
}
