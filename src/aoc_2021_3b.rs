use crate::aoc_2021_3::Reading;

#[cfg(test)]
mod tests;

pub fn determine_oxygen_co2_and_life_support_ratings(
    readings: impl IntoIterator<Item = Reading>,
) -> (u32, u32, u64) {
    let mut report = Report::new();
    for reading in readings {
        report.add_reading(reading)
    }
    (
        report.oxygen_rating(),
        report.co2_rating(),
        report.life_support_rating(),
    )
}

struct Report {
    bit_sums: [u32; 32],
    readings: Vec<Reading>,
}

impl Report {
    pub fn new() -> Self {
        Self {
            bit_sums: [0; 32],
            readings: Vec::new(),
        }
    }

    pub fn oxygen_rating(&self) -> u32 {
        let mut readings = self.readings.clone();
        for i in (0..32).rev() {
            if readings.len() == 1 {
                break;
            }
            let bits_set = readings
                .iter()
                .filter(|&&reading| reading.is_bit_set(i as u8))
                .count() as i64;
            if bits_set >= readings.len() as i64 - bits_set && bits_set > 0 {
                readings = readings
                    .into_iter()
                    .filter(|&reading| reading.is_bit_set(i as u8))
                    .collect();
            } else {
                readings = readings
                    .into_iter()
                    .filter(|&reading| !reading.is_bit_set(i as u8))
                    .collect();
            }
        }
        if !readings.is_empty() {
            readings[0].value()
        } else {
            0
        }
    }

    pub fn co2_rating(&self) -> u32 {
        let mut readings = self.readings.clone();
        for i in (0..32).rev() {
            if readings.len() == 1 {
                break;
            }
            let bits_set = readings
                .iter()
                .filter(|&&reading| reading.is_bit_set(i as u8))
                .count() as i64;
            if bits_set < readings.len() as i64 - bits_set && bits_set > 0 {
                readings = readings
                    .into_iter()
                    .filter(|&reading| reading.is_bit_set(i as u8))
                    .collect();
            } else {
                readings = readings
                    .into_iter()
                    .filter(|&reading| !reading.is_bit_set(i as u8))
                    .collect();
            }
        }
        if !readings.is_empty() {
            readings[0].value()
        } else {
            0
        }
    }

    pub fn add_reading(&mut self, reading: Reading) {
        for i in 0..32 {
            self.bit_sums[i] += reading.test(i as u8) as u32;
        }
        self.readings.push(reading);
    }

    fn life_support_rating(&self) -> u64 {
        self.oxygen_rating() as u64 * self.co2_rating() as u64
    }
}
