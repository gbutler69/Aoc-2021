use itertools::Itertools;
use rayon::prelude::*;

#[cfg(test)]
mod tests;

pub fn determine_number_of_fish_after_x_days(fish: Vec<u8>, days: u32) -> u64 {
    School::from(fish).population_after(days)
}

pub struct School {
    fish: Vec<Fish>,
}

impl School {
    fn population_after(&mut self, days: u32) -> u64 {
        for _ in 0..days {
            let new_fish = self.fish.par_iter_mut().map(|fish| fish.cycle_life()).sum();
            self.fish.push(Fish::from((8, new_fish)));
        }
        self.fish.iter().map(|f| f.num_fish).sum()
    }
}

impl From<Vec<u8>> for School {
    fn from(fish: Vec<u8>) -> Self {
        Self {
            fish: fish
                .into_iter()
                .sorted_by_key(|cycle| *cycle)
                .group_by(|cycle| *cycle)
                .into_iter()
                .map(|(key, values)| Fish::from((key, values.into_iter().count() as u64)))
                .collect::<Vec<_>>(),
        }
    }
}

pub struct Fish {
    cycle: u8,
    num_fish: u64,
}

impl Fish {
    fn cycle_life(&mut self) -> u64 {
        if self.cycle == 0 {
            self.cycle = 6;
            self.num_fish
        } else {
            self.cycle -= 1;
            0
        }
    }
}

impl From<(u8, u64)> for Fish {
    fn from((cycle, num_fish): (u8, u64)) -> Self {
        Self { cycle, num_fish }
    }
}
