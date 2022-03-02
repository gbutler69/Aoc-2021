use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub fn determine_number_of_paths_from_start_to_end(
    edges: impl IntoIterator<Item = (String, String)>,
) -> u32 {
    let mut map = CaveMap::default();
    for edge in edges {
        map.add(edge);
    }
    map.total_start_to_end_paths()
}

#[derive(Default, Debug)]
struct CaveMap {
    edges: HashMap<String, Vec<String>>,
}

impl CaveMap {
    fn add(&mut self, edge: (String, String)) {
        self.edges
            .entry(edge.0.clone())
            .and_modify(|v| v.push(edge.1.clone()))
            .or_insert_with(|| vec![edge.1.clone()]);
        self.edges
            .entry(edge.1)
            .and_modify(|v| v.push(edge.0.clone()))
            .or_insert_with(|| vec![edge.0.clone()]);
    }

    fn total_start_to_end_paths(&self) -> u32 {
        let mut path = imbl::Vector::new();
        self.count_paths_to_end("start".into(), &mut path)
    }

    fn count_paths_to_end(&self, vertex: String, path: &mut imbl::Vector<String>) -> u32 {
        if this_is_the_end_cave(&vertex) {
            return 1;
        }
        if this_is_a_repeated_visit_to_the_start_or_a_small_cave_and_not_the_only_repeated_visit_to_a_small_nonstart_cave(&vertex, path) {
            return 0;
        }
        match self.edges.get(&vertex) {
            Some(vertices) => self.total_subpaths_to_end_from_here(path, vertex, vertices),
            None => 0,
        }
    }

    #[allow(clippy::ptr_arg)]
    fn total_subpaths_to_end_from_here(
        &self,
        path: &mut imbl::Vector<String>,
        vertex: String,
        vertices: &Vec<String>,
    ) -> u32 {
        path.push_back(vertex);
        let total_subpaths = vertices
            .iter()
            .map(|vertice| self.count_paths_to_end(vertice.clone(), path))
            .sum();
        path.pop_back();
        total_subpaths
    }
}

#[allow(clippy::ptr_arg)]
fn this_is_the_end_cave(vertex: &String) -> bool {
    *vertex == "end"
}

#[allow(clippy::ptr_arg)]
fn this_is_a_repeated_visit_to_the_start_or_a_small_cave_and_not_the_only_repeated_visit_to_a_small_nonstart_cave(
    vertex: &String,
    path: &mut imbl::Vector<String>,
) -> bool {
    (this_is_the_second_visit_to_the_start_cave(vertex, path))
        || (this_is_a_small_cave_that_has_already_been_visited(vertex, path)
            && there_exists_an_already_repeated_small_cave(path))
}

#[allow(clippy::ptr_arg)]
fn this_is_the_second_visit_to_the_start_cave(
    vertex: &String,
    path: &mut imbl::Vector<String>,
) -> bool {
    this_is_the_start_cave(vertex) && this_cave_has_already_been_visited(path, vertex)
}

#[allow(clippy::ptr_arg)]
fn this_is_the_start_cave(vertex: &String) -> bool {
    vertex == "start"
}

#[allow(clippy::ptr_arg)]
fn this_is_a_small_cave_that_has_already_been_visited(
    vertex: &String,
    path: &mut imbl::Vector<String>,
) -> bool {
    this_is_a_small_cave(vertex) && this_cave_has_already_been_visited(path, vertex)
}

#[allow(clippy::ptr_arg)]
fn this_is_a_small_cave(vertex: &String) -> bool {
    ('a'..='z').contains(&vertex.chars().next().unwrap())
}

#[allow(clippy::ptr_arg)]
fn this_cave_has_already_been_visited(path: &mut imbl::Vector<String>, vertex: &String) -> bool {
    path.contains(vertex)
}

fn there_exists_an_already_repeated_small_cave(path: &mut imbl::Vector<String>) -> bool {
    for cave in path.iter().filter(|v| this_is_a_small_cave(*v)) {
        if path.iter().filter(|some_cave| **some_cave == *cave).count() > 1 {
            return true;
        }
    }
    false
}
