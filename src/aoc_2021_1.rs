use itertools::Itertools;

#[cfg(test)]
mod tests;

pub fn count_increased_measurements(measurements: impl IntoIterator<Item = u32>) -> u32 {
    measurements
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| if a < b { 1 } else { 0 })
        .sum()
}
