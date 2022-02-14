use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub fn determine_number_of_points_with_overlapping(
    vectors: impl IntoIterator<Item = Vector2D>,
) -> u64 {
    let mut points_with_intersected_counts = HashMap::new();
    for point in vectors
        .into_iter()
        .filter(|v| v.is_horizontal() || v.is_vertical())
        .flat_map(|v| v.points())
    {
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

#[derive(Copy, Clone)]
pub struct Vector2D {
    p1: Point,
    p2: Point,
}

impl Vector2D {
    pub fn points(&self) -> Vec<Point> {
        match self.orientation() {
            Orientation::Horizontal => self.horizontal_points(),
            Orientation::Vertical => self.vertical_points(),
            Orientation::Sloped => self.sloped_points(),
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    pub fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    fn orientation(&self) -> Orientation {
        match self.is_horizontal() {
            true => Orientation::Horizontal,
            false => match self.is_vertical() {
                true => Orientation::Vertical,
                false => Orientation::Sloped,
            },
        }
    }

    fn horizontal_points(&self) -> Vec<Point> {
        range(self.p1.x, self.p2.x)
            .map(|x| Point::from((x, self.p1.y)))
            .collect()
    }

    fn vertical_points(&self) -> Vec<Point> {
        range(self.p1.y, self.p2.y)
            .map(|y| Point::from((self.p1.x, y)))
            .collect()
    }

    fn sloped_points(&self) -> Vec<Point> {
        match (self.p1.x <= self.p2.x, self.p1.y <= self.p2.y) {
            (true, true) => self.sloped_points_up(),
            (true, false) => self.sloped_points_down(),
            (false, true) => self.sloped_points_up_reversed(),
            (false, false) => self.sloped_points_down_reversed(),
        }
    }

    fn sloped_points_up(&self) -> Vec<Point> {
        range(self.p1.x, self.p2.x)
            .enumerate()
            .map(|(i, x)| Point::from((x, self.p1.y + i as u32)))
            .collect()
    }

    fn sloped_points_down(&self) -> Vec<Point> {
        range(self.p1.x, self.p2.x)
            .enumerate()
            .map(|(i, x)| Point::from((x, self.p1.y - i as u32)))
            .collect()
    }

    fn sloped_points_up_reversed(&self) -> Vec<Point> {
        range(self.p1.x, self.p2.x)
            .enumerate()
            .map(|(i, x)| Point::from((x, self.p2.y - i as u32)))
            .collect()
    }

    fn sloped_points_down_reversed(&self) -> Vec<Point> {
        range(self.p1.x, self.p2.x)
            .enumerate()
            .map(|(i, x)| Point::from((x, self.p2.y + i as u32)))
            .collect()
    }
}

fn range(a: u32, b: u32) -> std::ops::RangeInclusive<u32> {
    if a <= b {
        a..=b
    } else {
        b..=a
    }
}

impl From<((u32, u32), (u32, u32))> for Vector2D {
    fn from(vertices: ((u32, u32), (u32, u32))) -> Self {
        Self {
            p1: Point::from(vertices.0),
            p2: Point::from(vertices.1),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Point {
    x: u32,
    y: u32,
}

impl From<(u32, u32)> for Point {
    fn from(point: (u32, u32)) -> Self {
        Point {
            x: point.0,
            y: point.1,
        }
    }
}

enum Orientation {
    Horizontal,
    Vertical,
    Sloped,
}
