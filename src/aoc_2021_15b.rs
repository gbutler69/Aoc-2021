use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub fn determine_danger_cost_of_least_cost_path_from_tile_of_5x5_overall_area(
    danger_values: Vec<Vec<u8>>,
) -> u32 {
    Cavern::from((danger_values, 5))
        .danger_cost_of_least_cost_path_to_end_from_start_using_dijkstra()
}

#[derive(Default, Debug)]
struct Cavern {
    danger_levels: Vec<Vec<u8>>,
    real_area_factor: usize,
}

impl Cavern {
    fn current_location_is_end(&self, col: usize, row: usize) -> bool {
        col == self.real_width() - 1 && row == self.real_height() - 1
    }

    fn tile_width(&self) -> usize {
        self.danger_levels[0].len()
    }

    fn tile_height(&self) -> usize {
        self.danger_levels.len()
    }

    fn real_width(&self) -> usize {
        self.tile_width() * self.real_area_factor
    }

    fn real_height(&self) -> usize {
        self.tile_height() * self.real_area_factor
    }

    fn danger_cost_of_least_cost_path_to_end_from_start_using_dijkstra(&self) -> u32 {
        let mut unvisited = HashMap::<(usize, usize), usize>::new();
        for row in 0..self.real_height() {
            for col in 0..self.real_width() {
                unvisited.insert((col, row), usize::MAX);
            }
        }
        unvisited.insert((0, 0), 0);

        let mut current_node = (0, 0);
        loop {
            let current_node_cost_from_start = *unvisited.get(&current_node).unwrap();
            if current_node_cost_from_start == usize::MAX {
                panic!("Unable to get there from here!")
            }
            if self.current_location_is_end(current_node.0, current_node.1) {
                return current_node_cost_from_start as u32;
            }
            self.update_neighbors_costs_from_start(
                current_node,
                current_node_cost_from_start,
                &mut unvisited,
            );
            unvisited.remove(&current_node);
            update_current_node_to_the_minimum_cost_unvisited_node(&mut current_node, &unvisited);
        }
    }

    fn update_neighbors_costs_from_start(
        &self,
        current_node: (usize, usize),
        current_node_cost_from_start: usize,
        unvisited: &mut HashMap<(usize, usize), usize>,
    ) {
        self.update_cost_to(
            Self::node_above,
            current_node,
            current_node_cost_from_start,
            unvisited,
        );
        self.update_cost_to(
            Self::node_below,
            current_node,
            current_node_cost_from_start,
            unvisited,
        );
        self.update_cost_to(
            Self::node_left,
            current_node,
            current_node_cost_from_start,
            unvisited,
        );
        self.update_cost_to(
            Self::node_right,
            current_node,
            current_node_cost_from_start,
            unvisited,
        );
    }

    #[allow(clippy::type_complexity)]
    fn update_cost_to(
        &self,
        node_selector: fn(&Self, (usize, usize)) -> Option<(usize, usize)>,
        current_node: (usize, usize),
        current_node_cost_from_start: usize,
        unvisited: &mut HashMap<(usize, usize), usize>,
    ) {
        if let Some((col, row)) = node_selector(self, current_node) {
            if unvisited.contains_key(&(col, row)) {
                let tile_col = col % self.tile_width();
                let tile_row = row % self.tile_height();
                let horizontal_tile = col / self.tile_width();
                let vertical_tile = row / self.tile_height();
                let cost_to_enter_node = (self.danger_levels[tile_row][tile_col] as usize
                    + horizontal_tile
                    + vertical_tile
                    - 1)
                    % 9
                    + 1;
                let cost_from_start = current_node_cost_from_start + cost_to_enter_node;
                unvisited
                    .entry((col, row))
                    .and_modify(|v| *v = (*v).min(cost_from_start));
            }
        }
    }

    fn node_above(&self, (col, row): (usize, usize)) -> Option<(usize, usize)> {
        if row > 0 {
            Some((col, row - 1))
        } else {
            None
        }
    }

    fn node_below(&self, (col, row): (usize, usize)) -> Option<(usize, usize)> {
        if row < self.real_height() - 1 {
            Some((col, row + 1))
        } else {
            None
        }
    }

    fn node_left(&self, (col, row): (usize, usize)) -> Option<(usize, usize)> {
        if col > 0 {
            Some((col - 1, row))
        } else {
            None
        }
    }

    fn node_right(&self, (col, row): (usize, usize)) -> Option<(usize, usize)> {
        if col < self.real_width() - 1 {
            Some((col + 1, row))
        } else {
            None
        }
    }
}

fn update_current_node_to_the_minimum_cost_unvisited_node(
    current_node: &mut (usize, usize),
    unvisited: &HashMap<(usize, usize), usize>,
) {
    *current_node = *unvisited
        .iter()
        .min_by_key(|((_, _), cost)| **cost)
        .unwrap()
        .0;
}

impl From<(Vec<Vec<u8>>, usize)> for Cavern {
    fn from((danger_levels, real_area_factor): (Vec<Vec<u8>>, usize)) -> Self {
        let cave_width = danger_levels[0].len();
        for row in danger_levels.iter() {
            assert_eq!(row.len(), cave_width)
        }
        Self {
            danger_levels,
            real_area_factor,
        }
    }
}
