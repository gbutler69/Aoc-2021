use rayon::prelude::*;

#[cfg(test)]
mod tests;

pub fn determine_number_of_fish_after_x_days(fish: Vec<Fish>, days: u32) -> u64 {
    School::from(fish).population_after(days)
}

pub struct School {
    fish: Vec<Fish>,
}

impl School {
    fn population_after(&mut self, days: u32) -> u64 {
        for _ in 0..days {
            let mut new_fish = self
                .fish
                .par_iter_mut()
                .flat_map(|fish| fish.cycle_life())
                .collect::<Vec<_>>();
            self.fish.append(&mut new_fish);
        }
        self.fish.len() as u64
    }
}

impl From<Vec<Fish>> for School {
    fn from(fish: Vec<Fish>) -> Self {
        Self { fish }
    }
}

pub struct Fish(u8);

impl Fish {
    fn cycle_life(&mut self) -> Option<Fish> {
        if self.0 == 0 {
            self.0 = 6;
            Some(Fish::from(8))
        } else {
            self.0 -= 1;
            None
        }
    }
}

impl From<u8> for Fish {
    fn from(cycle: u8) -> Self {
        Self(cycle)
    }
}
