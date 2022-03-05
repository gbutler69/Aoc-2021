use std::collections::HashMap;

use itertools::{Itertools, MinMaxResult};

#[cfg(test)]
mod tests;

pub fn determine_polymer_pairs_after_n_steps_given_rules_and_starting_polymer(
    rules: impl IntoIterator<Item = ((char, char), char)>,
    starting_polymer: impl IntoIterator<Item = char>,
    number_of_steps: usize,
) -> (HashMap<(char, char), usize>, char) {
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
        number_of_steps: usize,
    ) -> (HashMap<(char, char), usize>, char) {
        let mut polymer = HashMap::<(char, char), usize>::new();
        let mut last_element = ' ';
        for (left_element, right_element) in starting_polymer.into_iter().tuple_windows() {
            polymer
                .entry((left_element, right_element))
                .and_modify(|count| *count += 1 as usize)
                .or_insert(1);
            last_element = right_element;
        }
        for _ in 0..number_of_steps {
            polymer = {
                let mut modified_polymer = HashMap::with_capacity(polymer.len() * 2);
                for ((left_element, right_element), count) in polymer {
                    match self.rules.get(&(left_element, right_element)) {
                        Some(insertion) => {
                            modified_polymer
                                .entry((left_element, *insertion))
                                .and_modify(|c| *c += count)
                                .or_insert(count);
                            modified_polymer
                                .entry((*insertion, right_element))
                                .and_modify(|c| *c += count)
                                .or_insert(count);
                        }
                        None => {
                            modified_polymer
                                .entry((left_element, right_element))
                                .and_modify(|c| *c += count)
                                .or_insert(count);
                        }
                    }
                }
                modified_polymer
            }
        }
        (polymer, last_element)
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

pub fn greatest_minus_least_element_qty(polymer: &(HashMap<(char, char), usize>, char)) -> usize {
    let mut polymer_elements = HashMap::new();
    let (polymer, last_element) = polymer;
    polymer_elements.insert(*last_element, 1_usize);

    for ((left_element, _), count) in polymer.iter() {
        polymer_elements
            .entry(*left_element)
            .and_modify(|c| *c += count)
            .or_insert(*count);
    }

    let min_max = polymer_elements
        .into_iter()
        .minmax_by_key(|(_, count)| *count);

    match min_max {
        MinMaxResult::NoElements | MinMaxResult::OneElement(_) => 0,
        MinMaxResult::MinMax((_, min), (_, max)) => max - min,
    }
}
