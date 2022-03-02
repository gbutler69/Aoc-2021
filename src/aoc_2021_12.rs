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
        if vertex == "end" {
            return 1;
        }
        if ('a'..='z').contains(&vertex.chars().next().unwrap()) && path.contains(&vertex) {
            return 0;
        }
        match self.edges.get(&vertex) {
            Some(vertices) => {
                path.push_back(vertex);
                let total_subpaths = vertices
                    .iter()
                    .map(|vertice| self.count_paths_to_end(vertice.clone(), path))
                    .sum();
                path.pop_back();
                total_subpaths
            }
            None => 0,
        }
    }
}
