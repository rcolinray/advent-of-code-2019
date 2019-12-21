use crate::a_star::Searchable;
pub use crate::point::Point;

use std::collections::HashMap;

pub struct Grid<T> {
    nodes: HashMap<Point, T>,
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Grid {
            nodes: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn set(&mut self, point: Point, value: T) {
        self.nodes.insert(point, value);
    }

    pub fn get(&self, point: &Point) -> Option<&T> {
        self.nodes.get(point)
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.nodes.contains_key(point)
    }
}

impl<T> Searchable<Point> for Grid<T> {
    fn distance(&self, a: &Point, b: &Point) -> usize {
        a.distance_to(b)
    }

    fn get_neighbors(&self, point: &Point) -> Vec<Point> {
        let points = neighbors(*point);
        points
            .into_iter()
            .filter(|point| self.contains(point))
            .collect::<Vec<_>>()
    }
}

fn neighbors(point: Point) -> Vec<Point> {
    vec![
        point + Point::new(0, 1),
        point + Point::new(1, 0),
        point + Point::new(0, -1),
        point + Point::new(-1, 0),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        let points = neighbors(Point::new(0, 0));
        assert_eq!(
            points,
            [
                Point::new(0, 1),
                Point::new(1, 0),
                Point::new(0, -1),
                Point::new(-1, 0)
            ]
        );
    }

    #[test]
    fn test_graph() {
        let mut grid = Grid::new();
        grid.set(Point::new(0, 0), "a");
        grid.set(Point::new(0, 1), "b");
        grid.set(Point::new(1, 0), "c");
        grid.set(Point::new(0, -1), "d");
        grid.set(Point::new(-1, 0), "e");

        assert_eq!(grid.contains(&Point::new(0, 0)), true);
        assert_eq!(grid.contains(&Point::new(10, 10)), false);

        assert_eq!(
            grid.get_neighbors(&Point::new(0, 0)),
            vec![
                Point::new(0, 1),
                Point::new(1, 0),
                Point::new(0, -1),
                Point::new(-1, 0),
            ]
        );

        assert_eq!(grid.get_neighbors(&Point::new(10, 10)), vec![]);
    }
}
