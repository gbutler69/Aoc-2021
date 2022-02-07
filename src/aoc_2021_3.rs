#[cfg(test)]
mod tests;

pub fn determine_gamma_epsilon_and_power_factors(
    readings: impl IntoIterator<Item = Reading>,
) -> (u32, u32, u64) {
    let mut report = Report::new();
    for reading in readings {
        report.add_reading(reading)
    }
    (report.gamma(), report.epsilon(), report.power())
}

struct Report {
    bit_sums: [u32; 32],
    count: u32,
}

impl Report {
    pub fn new() -> Self {
        Self {
            bit_sums: [0; 32],
            count: 0,
        }
    }

    pub fn gamma(&self) -> u32 {
        let mut gamma = 0;
        for i in 0..32 {
            gamma += if self.bit_sums[i] as i64 > self.count as i64 - self.bit_sums[i] as i64
                && self.bit_sums[i] > 0
            {
                1 << i
            } else {
                0
            }
        }
        gamma
    }

    pub fn epsilon(&self) -> u32 {
        let mut epsilon = 0;
        for i in 0..32 {
            epsilon += if (self.bit_sums[i] as i64) < self.count as i64 - self.bit_sums[i] as i64
                && self.bit_sums[i] > 0
            {
                1 << i
            } else {
                0
            }
        }
        epsilon
    }

    pub fn add_reading(&mut self, reading: Reading) {
        self.count += 1;
        for i in 0..32 {
            self.bit_sums[i] += reading.test(i as u8) as u32;
        }
    }

    fn power(&self) -> u64 {
        self.gamma() as u64 * self.epsilon() as u64
    }
}

#[derive(Copy, Clone)]
pub struct Reading(u32);

impl Reading {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn test(&self, bit: u8) -> u32 {
        assert!(bit < 32);
        (self.0 >> bit) & 1
    }

    pub fn is_bit_set(&self, bit: u8) -> bool {
        self.test(bit) == 1
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl Default for Reading {
    fn default() -> Self {
        Self::new()
    }
}

impl From<u32> for Reading {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<&str> for Reading {
    fn from(bits: &str) -> Self {
        assert!(!bits.is_empty() && bits.len() <= 32);
        let reading = u32::from_str_radix(bits, 2).unwrap();
        Reading(reading)
    }
}
