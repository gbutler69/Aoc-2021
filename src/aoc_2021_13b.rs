use std::collections::HashSet;

#[cfg(test)]
mod tests;

pub fn determine_points_after_folds(
    points: impl IntoIterator<Item = (u32, u32)>,
    folds: impl IntoIterator<Item = (Axis, u32)>,
) -> Vec<Vec<char>> {
    PointsMap::from(points).apply(folds).printout()
}

#[derive(Default, Debug)]
struct PointsMap {
    points: HashSet<(u32, u32)>,
}

impl PointsMap {
    fn apply(&mut self, folds: impl IntoIterator<Item = (Axis, u32)>) -> &Self {
        for fold in folds {
            match fold.0 {
                Axis::Column => self.fold_along_column(fold.1),
                Axis::Row => self.fold_along_row(fold.1),
            };
        }
        self
    }

    fn printout(&self) -> Vec<Vec<char>> {
        let mut max_col = 0;
        let mut max_row = 0;
        for (col, row) in self.points.iter() {
            max_col = max_col.max(*col);
            max_row = max_row.max(*row);
        }
        let mut output = Vec::with_capacity(max_row as usize + 1);
        for row in 0..=max_row {
            let mut output_row = Vec::with_capacity(max_col as usize + 1);
            for col in 0..=max_col {
                output_row.push(if self.points.contains(&(col, row)) {
                    '#'
                } else {
                    ' '
                });
            }
            output.push(output_row);
        }
        output
    }

    fn fold_along_column(&mut self, fold_column: u32) {
        let reflected_points = {
            self.points
                .iter()
                .filter(|p| p.0 > fold_column)
                .map(|p| reflect_point_along_column(p, fold_column))
                .collect::<Vec<_>>()
        };
        self.remove_original_and_add_new_points_from(reflected_points);
    }

    fn fold_along_row(&mut self, fold_row: u32) {
        let reflected_points = {
            self.points
                .iter()
                .filter(|p| p.1 > fold_row)
                .map(|p| reflect_point_along_row(p, fold_row))
                .collect::<Vec<_>>()
        };
        self.remove_original_and_add_new_points_from(reflected_points);
    }

    fn remove_original_and_add_new_points_from(
        &mut self,
        reflected_points: Vec<((u32, u32), (u32, u32))>,
    ) {
        for point in reflected_points {
            self.points.remove(&point.0);
            self.points.insert(point.1);
        }
    }
}

#[allow(clippy::clone_on_copy)]
fn reflect_point_along_column(p: &(u32, u32), fold_column: u32) -> ((u32, u32), (u32, u32)) {
    (p.clone(), (2 * fold_column - p.0, p.1))
}

#[allow(clippy::clone_on_copy)]
fn reflect_point_along_row(p: &(u32, u32), fold_row: u32) -> ((u32, u32), (u32, u32)) {
    (p.clone(), (p.0, 2 * fold_row - p.1))
}

impl<Points> From<Points> for PointsMap
where
    Points: IntoIterator<Item = (u32, u32)>,
{
    fn from(points: Points) -> Self {
        Self {
            points: points.into_iter().collect::<HashSet<_>>(),
        }
    }
}

pub enum Axis {
    Column,
    Row,
}

impl From<&str> for Axis {
    fn from(axis: &str) -> Self {
        match axis {
            "x" => Axis::Column,
            _ => Axis::Row,
        }
    }
}
