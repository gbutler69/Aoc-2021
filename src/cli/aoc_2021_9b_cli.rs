use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use itertools::Itertools;

use aoc_2021::aoc_2021_9b::{determine_area_of_3_largest_basins_rank, HeightMap};

pub fn execute(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut height_map = HeightMap::default();
    let lines = io::BufReader::new(File::open(path)?)
        .lines()
        .filter_ok(|line| !line.trim().is_empty())
        .map_ok(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u8>())
                .collect::<Vec<_>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    for line in lines {
        height_map = height_map.add_row(line.into_iter().collect::<Result<Vec<_>, _>>()?);
    }
    let basin_area = determine_area_of_3_largest_basins_rank(height_map);
    println!("basin area {basin_area}");
    Ok(())
}
