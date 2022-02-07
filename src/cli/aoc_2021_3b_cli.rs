use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use itertools::Itertools;

use aoc_2021::aoc_2021_3::Reading;
use aoc_2021::aoc_2021_3b::determine_oxygen_co2_and_life_support_ratings;

pub fn execute(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let (oxygen_rating, co2_rating, life_support_rating) =
        determine_oxygen_co2_and_life_support_ratings(
            io::BufReader::new(File::open(path)?)
                .lines()
                .map_ok(|line| Reading::from(line.as_str()))
                .collect::<Result<Vec<_>, _>>()?,
        );
    println!("oxygen rating {oxygen_rating} co2 rating {co2_rating} life support rating {life_support_rating}");
    Ok(())
}
