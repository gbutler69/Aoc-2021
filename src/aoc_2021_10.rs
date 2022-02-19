use rayon::prelude::*;

#[cfg(test)]
mod tests;

pub fn determine_total_risk_level_of_low_points(height_map: HeightMap) -> u64 {
    height_map.risk()
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

    fn risk(&self) -> u64 {
        (0..self.width)
            .into_par_iter()
            .flat_map(|col| {
                (0..(self.map.len() / self.width))
                    .into_par_iter()
                    .map(move |row| (col, row))
            })
            .map(|(col, row)| self.risk_at(col, row))
            .sum()
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

    fn risk_at(&self, col: usize, row: usize) -> u64 {
        let level = self.map[row * self.width + col];
        let width = self.width;
        let height = self.map.len() / width;
        if (col == 0 || self.map[row * width + col - 1] > level)
            && (col == width - 1 || self.map[row * width + col + 1] > level)
            && (row == 0 || self.map[(row - 1) * width + col] > level)
            && (row == height - 1 || self.map[(row + 1) * width + col] > level)
        {
            (level + 1) as u64
        } else {
            0
        }
    }
}

impl Default for HeightMap {
    fn default() -> Self {
        Self::new()
    }
}
