use std::collections::HashMap;

use crate::aoc_2021_5::Vector2D;

#[cfg(test)]
mod tests;

pub fn determine_number_of_points_with_overlapping(
    vectors: impl IntoIterator<Item = Vector2D>,
) -> u64 {
    let mut points_with_intersected_counts = HashMap::new();
    for point in vectors.into_iter().flat_map(|v| v.points()) {
        points_with_intersected_counts
            .entry(point)
            .and_modify(|v| *v += 1_u32)
            .or_insert(1_u32);
    }
    points_with_intersected_counts
        .values()
        .filter(|v| **v > 1)
        .count() as u64
}
