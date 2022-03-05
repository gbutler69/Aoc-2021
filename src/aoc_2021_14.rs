use std::collections::HashMap;

use itertools::Itertools;

#[cfg(test)]
mod tests;

pub fn determine_polymer_after_n_steps_given_rules_and_starting_polymer(
    rules: impl IntoIterator<Item = ((char, char), char)>,
    starting_polymer: impl IntoIterator<Item = char>,
    number_of_steps: u32,
) -> Vec<char> {
    PolymerMutator::from(rules).mutate(starting_polymer, number_of_steps)
}

#[derive(Default, Debug)]
struct PolymerMutator {
    rules: HashMap<(char, char), char>,
}

impl PolymerMutator {
    fn mutate(
        &self,
        starting_polymer: impl IntoIterator<Item = char>,
        number_of_steps: u32,
    ) -> Vec<char> {
        let mut polymer = starting_polymer.into_iter().collect::<Vec<_>>();
        for _ in 0..number_of_steps {
            polymer = {
                let mut modified_polymer = Vec::with_capacity(polymer.len() * 2);
                for i in 0..(polymer.len() - 1) {
                    modified_polymer.push(polymer[i]);
                    if let Some(element) = self.rules.get(&(polymer[i], polymer[i + 1])) {
                        modified_polymer.push(*element);
                    }
                }
                modified_polymer.push(polymer[polymer.len() - 1]);
                modified_polymer
            }
        }
        polymer
    }
}

impl<Rules> From<Rules> for PolymerMutator
where
    Rules: IntoIterator<Item = ((char, char), char)>,
{
    fn from(rules: Rules) -> Self {
        Self {
            rules: rules.into_iter().collect::<HashMap<_, _>>(),
        }
    }
}

pub fn greatest_element_qty(polymer: &[char]) -> u32 {
    polymer
        .iter()
        .sorted()
        .group_by(|element| **element)
        .into_iter()
        .map(|(element, group)| (element, group.into_iter().count()))
        .max_by_key(|(_, count)| *count)
        .map_or(0, |(_, qty)| qty as u32)
}

pub fn least_element_qty(polymer: &[char]) -> u32 {
    polymer
        .iter()
        .sorted()
        .group_by(|element| **element)
        .into_iter()
        .map(|(element, group)| (element, group.into_iter().count()))
        .min_by_key(|(_, count)| *count)
        .map_or(0, |(_, qty)| qty as u32)
}
