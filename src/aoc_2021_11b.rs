use std::collections::HashSet;

use rayon::prelude::*;

#[cfg(test)]
mod tests;

pub fn determine_step_where_all_octopi_flash(
    starting_energy_levels: impl IntoIterator<Item = Vec<u8>>,
    number_of_steps: u16,
) -> Option<u32> {
    let mut octopi = Octopi::default();
    for group_of_levels in starting_energy_levels {
        octopi.add_group_of_levels(group_of_levels);
    }
    octopi.step_where_all_flash(number_of_steps)
}

#[derive(Default, Debug)]
struct Octopi {
    group_size: usize,
    energy_levels: Vec<u8>,
}

impl Octopi {
    fn step_where_all_flash(&mut self, number_of_steps: u16) -> Option<u32> {
        let total_octopi = self.energy_levels.len() as u32;
        (0..number_of_steps)
            .map(|i| (i, self.perform_step()))
            .find(|(_, n)| *n == total_octopi)
            .map(|(i, _)| i as u32 + 1)
    }

    fn add_group_of_levels(&mut self, mut group_of_levels: Vec<u8>) {
        if self.energy_levels.is_empty() {
            self.group_size = group_of_levels.len();
        } else {
            assert!(self.group_size == group_of_levels.len());
        }
        self.energy_levels.append(&mut group_of_levels);
    }

    fn perform_step(&mut self) -> u32 {
        self.increment_all_energy_levels();
        let mut flashed = HashSet::new();
        loop {
            let flashes = self.perform_round_of_flashes(&mut flashed);
            if flashes == 0 {
                break;
            }
        }
        self.reset_energy_levels_of_flashed();
        flashed.len() as u32
    }

    fn perform_round_of_flashes(&mut self, already_flashed: &mut HashSet<(usize, usize)>) -> u32 {
        let width = self.group_size;
        let height = self.energy_levels.len() / width;
        let mut flashed = HashSet::new();
        for col in 0..width {
            for row in 0..height {
                if self.energy_levels[col + row * width] > 9
                    && !already_flashed.contains(&(col, row))
                {
                    flashed.insert((col, row));
                }
            }
        }
        self.increment_all_adjacent_energy_levels(&flashed);
        already_flashed.extend(flashed.iter());
        flashed.len() as u32
    }

    fn increment_all_energy_levels(&mut self) {
        self.energy_levels.par_iter_mut().for_each(|el| *el += 1);
    }

    fn reset_energy_levels_of_flashed(&mut self) {
        self.energy_levels
            .par_iter_mut()
            .filter(|el| **el > 9)
            .for_each(|el| *el = 0);
    }

    fn increment_all_adjacent_energy_levels(&mut self, flashed: &HashSet<(usize, usize)>) {
        flashed
            .iter()
            .for_each(|(col, row)| self.increment_adjacent_energy_levels(*col, *row))
    }

    fn increment_adjacent_energy_levels(&mut self, col: usize, row: usize) {
        self.increment_top_left_energy_level(col, row);
        self.increment_top_energy_level(col, row);
        self.increment_top_right_energy_level(col, row);
        self.increment_left_energy_level(col, row);
        self.increment_right_energy_level(col, row);
        self.increment_bottom_left_energy_level(col, row);
        self.increment_bottom_energy_level(col, row);
        self.increment_bottom_right_energy_level(col, row);
    }

    fn increment_top_left_energy_level(&mut self, col: usize, row: usize) {
        if col > 0 && row > 0 {
            self.energy_levels[(col - 1) + (row - 1) * self.group_size] += 1;
        }
    }

    fn increment_top_energy_level(&mut self, col: usize, row: usize) {
        if row > 0 {
            self.energy_levels[col + (row - 1) * self.group_size] += 1;
        }
    }

    fn increment_top_right_energy_level(&mut self, col: usize, row: usize) {
        if col < self.max_col() && row > 0 {
            self.energy_levels[(col + 1) + (row - 1) * self.group_size] += 1;
        }
    }

    fn increment_left_energy_level(&mut self, col: usize, row: usize) {
        if col > 0 {
            self.energy_levels[(col - 1) + row * self.group_size] += 1;
        }
    }

    fn increment_right_energy_level(&mut self, col: usize, row: usize) {
        if col < self.group_size - 1 {
            self.energy_levels[(col + 1) + row * self.group_size] += 1;
        }
    }

    fn increment_bottom_left_energy_level(&mut self, col: usize, row: usize) {
        if col > 0 && row < self.max_row() {
            self.energy_levels[(col - 1) + (row + 1) * self.group_size] += 1;
        }
    }

    fn increment_bottom_energy_level(&mut self, col: usize, row: usize) {
        if row < self.max_row() {
            self.energy_levels[col + (row + 1) * self.group_size] += 1;
        }
    }

    fn increment_bottom_right_energy_level(&mut self, col: usize, row: usize) {
        if col < self.max_col() && row < self.max_row() {
            self.energy_levels[(col + 1) + (row + 1) * self.group_size] += 1;
        }
    }

    fn max_row(&self) -> usize {
        self.energy_levels.len() / self.group_size - 1
    }

    fn max_col(&mut self) -> usize {
        self.group_size - 1
    }
}
