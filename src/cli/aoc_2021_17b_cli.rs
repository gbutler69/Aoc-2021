use std::{
    fs::File,
    io::{self, BufRead},
    ops::RangeInclusive,
    path::PathBuf,
};

use aoc_2021::aoc_2021_17b::determine_all_trajectories_that_hit_target_area;

pub fn execute(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    let _ = io::BufReader::new(File::open(path)?).read_line(&mut buf)?;
    let x_range = parse_x_range(&buf);
    let y_range = parse_y_range(&buf);
    let num_trajectories = determine_all_trajectories_that_hit_target_area(x_range, y_range).len();
    println!("number of successful trajectories {num_trajectories}",);
    Ok(())
}

fn parse_x_range(buf: &str) -> RangeInclusive<i64> {
    let xr = buf
        .strip_prefix("target area: x=")
        .expect("input must begin with \"target area: x=\"")
        .split_once(",")
        .expect("input must include a comma (,) between the x and y ranges")
        .0
        .split_once("..")
        .expect("x-range must be formatted as N..M");
    RangeInclusive::new(
        xr.0.parse()
            .expect("x-range must be formatted as N..M where N is a valid integer"),
        xr.1.parse()
            .expect("x-range must be formatted as N..M where M is a valid integer"),
    )
}

fn parse_y_range(buf: &str) -> RangeInclusive<i64> {
    let xr = buf
        .split_once(", y=")
        .expect("r-range must come after comma and space (, ) and be formatted as y=P..Q\"")
        .1
        .strip_suffix('\n')
        .expect("there must be a new-line (LF) at the end of the input")
        .split_once("..")
        .expect("y-range is formatted as P..Q\"");
    RangeInclusive::new(
        xr.0.parse()
            .expect("y-range is formatted as P..Q where P is a valid integer"),
        xr.1.parse()
            .expect("y-range is formatted as P..Q where Q is a valid integer"),
    )
}
