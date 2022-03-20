use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use aoc_2021::aoc_2021_16b::determine_numeric_result_of_packet_evaluation;

pub fn execute(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    let _ = io::BufReader::new(File::open(path)?).read_line(&mut buf)?;
    let bytes = hex::decode(&buf[0..buf.len() - 1])?;
    let value = determine_numeric_result_of_packet_evaluation(bytes);
    println!("evaluated value of packet {value}");
    Ok(())
}
