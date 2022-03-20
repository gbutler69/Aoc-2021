use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use aoc_2021::aoc_2021_16::determine_packet_and_subpacket_version_numbers;

pub fn execute(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    let _ = io::BufReader::new(File::open(path)?).read_line(&mut buf)?;
    let bytes = hex::decode(&buf[0..buf.len() - 1])?;
    let packet_versions = determine_packet_and_subpacket_version_numbers(bytes);
    println!(
        "sum of packet versions {sum}",
        sum = packet_versions.into_iter().map(u64::from).sum::<u64>()
    );
    Ok(())
}
