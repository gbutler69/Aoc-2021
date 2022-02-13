use crate::aoc_2021_4::Board;
use crate::aoc_2021_4::Game;

#[cfg(test)]
mod tests;

pub fn determine_last_winning_board_uncalled_total_last_called_number_and_score(
    boards: impl IntoIterator<Item = Board>,
    numbers_called: impl IntoIterator<Item = u8>,
) -> (Option<u8>, Option<u32>, Option<u8>, Option<u64>) {
    let game = Game::new(boards, numbers_called);
    (
        game.last_winning_board(),
        game.last_wb_uncalled_total(),
        game.last_wb_last_called_number(),
        game.last_wb_score(),
    )
}
