use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use itertools::Itertools;

use aoc_2021::aoc_2021_4::{
    determine_winning_board_uncalled_total_last_called_number_and_score, Board,
};

pub fn execute(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut input_lines = io::BufReader::new(File::open(path)?)
        .lines()
        .filter_ok(|l| !l.trim().is_empty());
    let numbers_called = input_lines
        .next()
        .ok_or("")??
        .split(',')
        .map(|v| v.parse::<u64>().unwrap() as u8)
        .collect::<Vec<_>>();
    let boards = input_lines
        .collect::<Result<Vec<_>, _>>()?
        .chunks(5)
        .map(|c| {
            Board::from(
                c.iter()
                    .flat_map(|l| {
                        l.split_whitespace()
                            .map(|s| s.parse::<u64>().unwrap() as u8)
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    match determine_winning_board_uncalled_total_last_called_number_and_score(
        boards,
        numbers_called,
    ) {
        (Some(wb), Some(ut), Some(lcn), Some(s)) => {
            println!("winning board {wb} uncalled total {ut} last called number {lcn} score {s}")
        }
        _ => todo!(),
    };
    Ok(())
}
