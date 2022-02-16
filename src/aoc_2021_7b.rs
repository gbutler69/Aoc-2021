use std::collections::HashMap;

use rayon::prelude::*;

#[cfg(test)]
mod tests;

pub fn determine_position_and_minimum_fuel_required(positions: Vec<u32>) -> (u32, u64) {
    if positions.is_empty() {
        return (0, 0);
    }
    let min_position = *positions.iter().min().unwrap();
    let max_position = *positions.iter().max().unwrap();
    let mut fuel_costs = HashMap::<u32, u64>::new();
    for position in min_position..=max_position {
        fuel_costs.insert(
            position,
            fuel_cost_to_target_for_positions(position, &positions),
        );
    }
    fuel_costs.into_iter().min_by_key(|(_, v)| *v).unwrap()
}

fn fuel_cost_to_target_for_positions(target: u32, positions: &[u32]) -> u64 {
    positions
        .par_iter()
        .map(|position| fuel_cost_to_target_for_position(target, *position))
        .sum()
}

fn fuel_cost_to_target_for_position(target: u32, position: u32) -> u64 {
    (1..=((target as i64 - position as i64).abs() as u64)).sum()
}
