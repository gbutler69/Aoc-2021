use itertools::Itertools;

#[cfg(test)]
mod tests;

pub fn determine_winning_board_uncalled_total_last_called_number_and_score(
    boards: impl IntoIterator<Item = Board>,
    numbers_called: impl IntoIterator<Item = u8>,
) -> (Option<u8>, Option<u32>, Option<u8>, Option<u64>) {
    let game = Game::new(boards, numbers_called);
    (
        game.winning_board(),
        game.uncalled_total(),
        game.last_called_number(),
        game.score(),
    )
}

pub(crate) struct Game {
    winning_board: Option<u8>,
    uncalled_total: Option<u32>,
    last_called_number: Option<u8>,
    score: Option<u64>,
    last_winning_board: Option<u8>,
    last_wb_uncalled_total: Option<u32>,
    last_wb_last_called_number: Option<u8>,
    last_wb_score: Option<u64>,
}

impl Game {
    pub fn new(
        boards: impl IntoIterator<Item = Board>,
        numbers_called: impl IntoIterator<Item = u8>,
    ) -> Self {
        let mut winner_boards = Vec::<(usize, usize, u32)>::new();
        let numbers_called = numbers_called.into_iter().collect::<Vec<_>>();
        for (i, board) in boards.into_iter().enumerate() {
            if let Some((winning_number_idx, unmarked_total)) = board.play(&numbers_called) {
                winner_boards.push((i, winning_number_idx, unmarked_total));
            }
        }
        match winner_boards
            .into_iter()
            .minmax_by_key(|(_, winning_number_idx, _)| *winning_number_idx)
        {
            itertools::MinMaxResult::NoElements => Self {
                winning_board: None,
                uncalled_total: None,
                last_called_number: None,
                score: None,
                last_winning_board: None,
                last_wb_uncalled_total: None,
                last_wb_last_called_number: None,
                last_wb_score: None,
            },
            itertools::MinMaxResult::OneElement((board, winning_number_idx, unmarked_total)) => {
                let last_called_number = numbers_called[winning_number_idx];
                Self {
                    winning_board: Some(board as u8 + 1),
                    uncalled_total: Some(unmarked_total),
                    last_called_number: Some(last_called_number),
                    score: Some(unmarked_total as u64 * last_called_number as u64),
                    last_winning_board: Some(board as u8 + 1),
                    last_wb_uncalled_total: Some(unmarked_total),
                    last_wb_last_called_number: Some(last_called_number),
                    last_wb_score: Some(unmarked_total as u64 * last_called_number as u64),
                }
            }
            itertools::MinMaxResult::MinMax(
                (board, winning_number_idx, unmarked_total),
                (l_board, l_winning_number_idx, l_unmarked_total),
            ) => {
                let last_called_number = numbers_called[winning_number_idx];
                let l_last_called_number = numbers_called[l_winning_number_idx];
                Self {
                    winning_board: Some(board as u8 + 1),
                    uncalled_total: Some(unmarked_total),
                    last_called_number: Some(last_called_number),
                    score: Some(unmarked_total as u64 * last_called_number as u64),
                    last_winning_board: Some(l_board as u8 + 1),
                    last_wb_uncalled_total: Some(l_unmarked_total),
                    last_wb_last_called_number: Some(l_last_called_number),
                    last_wb_score: Some(l_unmarked_total as u64 * l_last_called_number as u64),
                }
            }
        }
    }

    pub fn winning_board(&self) -> Option<u8> {
        self.winning_board
    }

    pub fn uncalled_total(&self) -> Option<u32> {
        self.uncalled_total
    }

    pub fn last_called_number(&self) -> Option<u8> {
        self.last_called_number
    }

    pub fn score(&self) -> Option<u64> {
        self.score
    }
    pub fn last_winning_board(&self) -> Option<u8> {
        self.last_winning_board
    }

    pub fn last_wb_uncalled_total(&self) -> Option<u32> {
        self.last_wb_uncalled_total
    }

    pub fn last_wb_last_called_number(&self) -> Option<u8> {
        self.last_wb_last_called_number
    }

    pub fn last_wb_score(&self) -> Option<u64> {
        self.last_wb_score
    }
}

pub struct Board([Option<u8>; 25]);

impl Board {
    fn play(mut self, numbers_called: &[u8]) -> Option<(usize, u32)> {
        for (number_called_idx, num) in numbers_called.iter().enumerate() {
            for marked_at in self.play_number(*num) {
                if self.winner(marked_at) {
                    return Some((number_called_idx, self.total_unmarked()));
                }
            }
        }
        None
    }

    fn play_number(&mut self, num: u8) -> Vec<(u8, u8)> {
        let mut marked_spaces = Vec::new();
        for (idx, n) in self
            .0
            .iter_mut()
            .enumerate()
            .filter(|(_, o)| o.is_some())
            .filter(|(_, o)| o.unwrap() == num)
        {
            *n = None;
            marked_spaces.push(((idx % 5) as u8, (idx / 5) as u8))
        }
        marked_spaces
    }

    fn winner(&self, marked_at: (u8, u8)) -> bool {
        self.is_column_a_winner(marked_at.0) || self.is_row_a_winner(marked_at.1)
    }

    fn total_unmarked(&self) -> u32 {
        self.0
            .iter()
            .filter(|o| o.is_some())
            .map(|o| o.unwrap() as u32)
            .sum()
    }

    fn is_column_a_winner(&self, col: u8) -> bool {
        self.0
            .iter()
            .skip(col as usize)
            .step_by(5)
            .filter(|o| o.is_none())
            .count()
            == 5
    }

    fn is_row_a_winner(&self, row: u8) -> bool {
        self.0
            .iter()
            .skip(row as usize * 5)
            .take(5)
            .filter(|o| o.is_none())
            .count()
            == 5
    }
}

impl From<[u8; 25]> for Board {
    fn from(values: [u8; 25]) -> Self {
        let mut s = Self([None; 25]);
        for (i, &v) in values.iter().enumerate() {
            s.0[i] = Some(v);
        }
        s
    }
}

impl From<Vec<u8>> for Board {
    fn from(values: Vec<u8>) -> Self {
        assert_eq!(25, values.len());
        let mut s = Self([None; 25]);
        for (i, &v) in values.iter().take(25).enumerate() {
            s.0[i] = Some(v);
        }
        s
    }
}
