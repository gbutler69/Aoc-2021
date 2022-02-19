use std::collections::HashSet;

use itertools::Itertools;
use rayon::prelude::*;

#[cfg(test)]
mod tests;

pub fn determine_area_of_3_largest_basins_rank(height_map: HeightMap) -> u64 {
    height_map
        .basin_sizes()
        .into_iter()
        .sorted()
        .rev()
        .take(3)
        .product()
}

pub struct HeightMap {
    map: Vec<u8>,
    width: usize,
}

impl HeightMap {
    fn new() -> Self {
        Self {
            map: Vec::new(),
            width: 0,
        }
    }

    pub fn add_row(mut self, mut vec: Vec<u8>) -> Self {
        if self.map.is_empty() {
            self.width = vec.len();
        } else {
            assert!(self.width == vec.len())
        }
        self.map.append(&mut vec);
        self
    }

    fn is_low_point(&self, col: usize, row: usize) -> bool {
        let level = self.map[row * self.width + col];
        let width = self.width;
        let height = self.map.len() / width;
        (col == 0 || self.map[row * width + col - 1] > level)
            && (col == width - 1 || self.map[row * width + col + 1] > level)
            && (row == 0 || self.map[(row - 1) * width + col] > level)
            && (row == height - 1 || self.map[(row + 1) * width + col] > level)
    }

    fn basin_sizes(&self) -> Vec<u64> {
        (0..self.width)
            .into_par_iter()
            .flat_map(|col| {
                (0..(self.map.len() / self.width))
                    .into_par_iter()
                    .map(move |row| (col, row))
            })
            .filter(|(col, row)| self.is_low_point(*col, *row))
            .map(|(col, row)| self.basin_size_at(col, row))
            .collect()
    }

    fn basin_size_at(&self, col: usize, row: usize) -> u64 {
        let level = self.map[row * self.width + col];
        let width = self.width;
        let height = self.map.len() / width;
        let mut explored = HashSet::new();
        explored.insert((col, row));

        let basin_size_above = if row == 0 {
            0
        } else {
            self.basin_size_at_level(col, row - 1, level, &mut explored)
        };

        let basin_size_below = if row == height - 1 {
            0
        } else {
            self.basin_size_at_level(col, row + 1, level, &mut explored)
        };

        let basin_size_left = if col == 0 {
            0
        } else {
            self.basin_size_at_level(col - 1, row, level, &mut explored)
        };

        let basin_size_right = if col == width - 1 {
            0
        } else {
            self.basin_size_at_level(col + 1, row, level, &mut explored)
        };

        1 + basin_size_above + basin_size_below + basin_size_left + basin_size_right
    }

    fn basin_size_at_level(
        &self,
        col: usize,
        row: usize,
        prev_level: u8,
        explored: &mut HashSet<(usize, usize)>,
    ) -> u64 {
        let level = self.map[row * self.width + col];
        let width = self.width;
        let height = self.map.len() / width;
        if level >= 9 || level <= prev_level || explored.contains(&(col, row)) {
            0
        } else {
            explored.insert((col, row));
            let basin_size_above = if row == 0 {
                0
            } else {
                self.basin_size_at_level(col, row - 1, level, explored)
            };

            let basin_size_below = if row == height - 1 {
                0
            } else {
                self.basin_size_at_level(col, row + 1, level, explored)
            };

            let basin_size_left = if col == 0 {
                0
            } else {
                self.basin_size_at_level(col - 1, row, level, explored)
            };

            let basin_size_right = if col == width - 1 {
                0
            } else {
                self.basin_size_at_level(col + 1, row, level, explored)
            };

            1 + basin_size_above + basin_size_below + basin_size_left + basin_size_right
        }
    }
}

impl Default for HeightMap {
    fn default() -> Self {
        Self::new()
    }
}
