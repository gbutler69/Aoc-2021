use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub fn determine_danger_cost_of_least_cost_path(danger_values: Vec<Vec<u8>>) -> u32 {
    Cavern::from(danger_values).danger_cost_of_least_cost_path_to_end_from_start_using_dijkstra()
}

#[derive(Default, Debug)]
struct Cavern {
    danger_levels: Vec<Vec<u8>>,
}

impl Cavern {
    fn current_location_is_end(&self, col: usize, row: usize) -> bool {
        col == self.width() - 1 && row == self.height() - 1
    }

    fn width(&self) -> usize {
        self.danger_levels[0].len()
    }

    fn height(&self) -> usize {
        self.danger_levels.len()
    }

    fn danger_cost_of_least_cost_path_to_end_from_start_using_dijkstra(&self) -> u32 {
        let mut unvisited = HashMap::<(usize, usize), usize>::new();
        for row in 0..self.height() {
            for col in 0..self.width() {
                unvisited.insert((col, row), usize::MAX);
            }
        }
        unvisited.insert((0, 0), 0);

        let mut current_node = (0, 0);
        loop {
            let current_node_cost_from_start = *unvisited.get(&current_node).unwrap();
            if self.current_location_is_end(current_node.0, current_node.1) {
                return current_node_cost_from_start as u32;
            }
            self.update_neighbors_cost_from_start(
                current_node,
                current_node_cost_from_start,
                &mut unvisited,
            );
            unvisited.remove(&current_node);
            current_node = *unvisited
                .iter()
                .min_by_key(|((_, _), cost)| **cost)
                .unwrap()
                .0;
        }
    }

    fn update_neighbors_cost_from_start(
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
                let cost_to_enter_node = self.danger_levels[row][col] as usize;
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
        if row < self.height() - 1 {
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
        if col < self.width() - 1 {
            Some((col + 1, row))
        } else {
            None
        }
    }
}

impl From<Vec<Vec<u8>>> for Cavern {
    fn from(danger_levels: Vec<Vec<u8>>) -> Self {
        let cave_width = danger_levels[0].len();
        for row in danger_levels.iter() {
            assert_eq!(row.len(), cave_width)
        }
        Self { danger_levels }
    }
}
