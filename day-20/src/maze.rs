use crate::grid::Grid;
use crate::path_cache::PathCache;
use crate::point::Point2D;
use crate::search::{a_star_search, djikstra_search, Searchable};

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

#[derive(Clone, Copy, Debug)]
pub enum Block {
    Wall,
    Empty,
}

impl Block {
    fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::Wall
    }
}

impl Grid<Block> {
    pub fn find_path(&self, begin: &Point2D, end: &Point2D) -> Option<Vec<Point2D>> {
        djikstra_search(begin, end, self)
    }

    fn get_empty_neighbors(&self, point: &Point2D) -> Vec<Point2D> {
        self.get_neighbors(point)
            .into_iter()
            .filter(|point| self.get(point).is_empty())
            .collect::<Vec<_>>()
    }
}

impl Searchable<Point2D> for Grid<Block> {
    fn get_neighbors(&self, current: &Point2D) -> Vec<Point2D> {
        self.get_empty_neighbors(current)
    }
}

pub struct Maze {
    grid: Grid<Block>,
    entrance: Point2D,
    exit: Point2D,
    portals: HashMap<Point2D, Point2D>,
    portal_names: HashMap<Point2D, String>,
}

const PADDING: usize = 2;
const ENTRANCE: &str = "AA";
const EXIT: &str = "ZZ";

impl Maze {
    pub fn from_file(filename: &str) -> Self {
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        Maze::from_string(&contents)
    }

    pub fn from_string(contents: &str) -> Self {
        let lines = contents
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let height = lines.len();
        let width = lines[0].len();
        let mut grid = Grid::new(width, height);
        let mut partial_portals = HashMap::<String, Vec<Point2D>>::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, &c) in line.iter().enumerate() {
                let point = Point2D::new(x, y);
                if c == '.' {
                    grid.set(&point, Block::Empty);
                } else if c.is_alphabetic() {
                    let neighbors = point.get_neighbors(width, height);
                    let mut portal = neighbors
                        .into_iter()
                        .filter_map(|point| {
                            let c = lines[point.y][point.x];
                            if c == '.' || c.is_alphabetic() {
                                Some((point, c))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();

                    if portal.len() != 2 {
                        continue;
                    }

                    portal.sort_by(|(point1, _), (point2, _)| point1.cmp(point2));

                    let first = portal[0];
                    let mut name = String::new();
                    let location = if first.1 == '.' {
                        name.push(c);
                        name.push(portal[1].1);
                        first.0
                    } else {
                        name.push(portal[0].1);
                        name.push(c);
                        portal[1].0
                    };

                    let point = Point2D::new(location.x, location.y);
                    if let Some(points) = partial_portals.get_mut(&name) {
                        points.push(point);
                    } else {
                        partial_portals.insert(name, vec![point]);
                    }
                }
            }
        }

        let entrance = partial_portals.remove(ENTRANCE).unwrap()[0];
        let exit = partial_portals.remove(EXIT).unwrap()[0];

        let mut portals = HashMap::new();
        let mut portal_names = HashMap::new();
        for portal in partial_portals.keys() {
            let points = partial_portals.get(portal).unwrap();
            portals.insert(points[0], points[1]);
            portals.insert(points[1], points[0]);
            portal_names.insert(points[0], portal.clone());
            portal_names.insert(points[1], portal.clone());
        }

        Maze {
            grid,
            entrance,
            exit,
            portals,
            portal_names,
        }
    }

    pub fn get_entrance(&self) -> &Point2D {
        &self.entrance
    }

    pub fn get_exit(&self) -> &Point2D {
        &self.exit
    }

    pub fn get_portals(&self) -> Vec<&Point2D> {
        self.portals.keys().collect::<Vec<_>>()
    }

    pub fn get_pair_portal(&self, point: &Point2D) -> Option<&Point2D> {
        self.portals.get(point)
    }

    pub fn get_path(&self, start: &Point2D, finish: &Point2D) -> Option<Vec<Point2D>> {
        djikstra_search(start, finish, self)
    }

    pub fn find_shortest_path_len(&self) -> Option<usize> {
        djikstra_search(self.get_entrance(), self.get_exit(), self).map(|path| path.len() - 1)
    }

    fn build_path_cache(&self) -> PathCache {
        PathCache::new(&self.grid, &self.entrance, &self.exit, &self.portals)
    }

    pub fn get_name(&self, point: &Point2D) -> Option<String> {
        self.portal_names.get(point).cloned().or_else(|| {
            if *point == self.entrance {
                Some("AA".to_owned())
            } else if *point == self.exit {
                Some("ZZ".to_owned())
            } else {
                None
            }
        })
    }
}

impl Searchable<Point2D> for Maze {
    fn get_neighbors(&self, current: &Point2D) -> Vec<Point2D> {
        let mut neighbors = self.grid.get_empty_neighbors(current);
        if let Some(destination) = self.portals.get(current) {
            neighbors.push(*destination);
        }
        neighbors
    }
}

pub struct RecursiveMaze {
    maze: Rc<Maze>,
    path_cache: Rc<PathCache>,
}

impl RecursiveMaze {
    pub fn from_file(filename: &str) -> Self {
        let maze = Maze::from_file(filename);
        let path_cache = maze.build_path_cache();

        Self::new(Rc::new(maze), Rc::new(path_cache))
    }

    pub fn new(maze: Rc<Maze>, path_cache: Rc<PathCache>) -> Self {
        Self { maze, path_cache }
    }

    fn is_outer_edge(&self, point: &Point2D) -> bool {
        point.x == PADDING
            || point.x == self.maze.grid.get_width() - PADDING - 1
            || point.y == PADDING
            || point.y == self.maze.grid.get_height() - PADDING - 1
    }

    pub fn find_path(&self) -> Option<Vec<RecursiveState>> {
        let initial_state = (*self.maze.get_entrance(), 0, "AA".to_owned());
        let final_state = (*self.maze.get_exit(), 0, "ZZ".to_owned());

        a_star_search(&initial_state, &final_state, self, |current, goal| {
            // If on same level, check for path to exit
            if current.1 == goal.1 {
                if let Some(path) = self.path_cache.get_path(&current.0, &goal.0) {
                    return path.len() - 1;
                }
            }

            // If on level below, check for path to exit from pair on level above
            if current.1 == 1 && self.is_outer_edge(&current.0) {
                if let Some(pair_portal) = self.maze.get_pair_portal(&current.0) {
                    if let Some(path) = self.path_cache.get_path(pair_portal, &goal.0) {
                        return path.len();
                    }
                }
            }

            // Assume 100 steps per level
            100 * (current.1 + 1)
        })
    }

    pub fn find_shortest_path_len(&self) -> Option<usize> {
        if let Some(path) = self.find_path() {
            let mut steps = 0;
            let mut current = &path[0];
            for next in &path[1..] {
                let len = self.distance(current, next);
                steps += len;

                if let Some(next_name) = self.maze.get_name(&next.0) {
                    if current.1 > next.1 {
                        println!(
                            "Return to level {} through {} ({} step)",
                            next.1, next_name, len
                        );
                    } else if current.1 < next.1 {
                        println!(
                            "Recurse into level {} through {} ({} step)",
                            current.1, next_name, len
                        );
                    } else if let Some(curr_name) = self.maze.get_name(&current.0) {
                        println!("Walk from {} to {} ({} steps)", curr_name, next_name, len);
                    }
                }

                current = next;
            }
            Some(steps)
        } else {
            None
        }
    }
}

type RecursiveState = (Point2D, usize, String);

impl Searchable<RecursiveState> for RecursiveMaze {
    fn distance(&self, current: &RecursiveState, neighbor: &RecursiveState) -> usize {
        if current.1 != neighbor.1 {
            1
        } else {
            self.path_cache
                .get_path(&current.0, &neighbor.0)
                .expect(&format!(
                    "Failed to get path from {:?} to {:?}",
                    current.0, neighbor.0
                ))
                .len()
                - 1
        }
    }

    fn get_neighbors(&self, current: &RecursiveState) -> Vec<RecursiveState> {
        let mut neighbors = Vec::new();

        if current.1 == 0 {
            let exit = self.maze.get_exit();
            if self.path_cache.has_path(&current.0, exit) {
                // Walk to the exit
                neighbors.push((*exit, current.1, "ZZ".to_owned()));
            } else if let Some(pair_portal) = self.maze.get_pair_portal(&current.0) {
                // Step through current portal
                neighbors.push((
                    *pair_portal,
                    1,
                    self.maze.get_name(pair_portal).unwrap().clone(),
                ));
            }

            neighbors.extend(
                self.maze
                    .get_portals()
                    .into_iter()
                    .filter(|point| {
                        !self.is_outer_edge(point) && self.path_cache.has_path(&current.0, point)
                    })
                    .map(|point| (*point, current.1, self.maze.get_name(point).unwrap())),
            );
        } else {
            if let Some(pair_portal) = self.maze.get_pair_portal(&current.0) {
                // Step through current portal
                let next_z = if self.is_outer_edge(&current.0) {
                    current.1 - 1
                } else {
                    current.1 + 1
                };
                neighbors.push((
                    *pair_portal,
                    next_z,
                    self.maze.get_name(pair_portal).unwrap().clone(),
                ));
            }

            // Walk from current point to reachable neighbor
            neighbors.extend(
                self.maze
                    .get_portals()
                    .into_iter()
                    .filter(|point| self.path_cache.has_path(&current.0, point))
                    .map(|point| {
                        (
                            *point,
                            current.1,
                            self.maze.get_name(point).unwrap().clone(),
                        )
                    }),
            );
        }

        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_example(filename: &str) -> usize {
        let maze = Maze::from_file(filename);
        maze.find_shortest_path_len().unwrap()
    }

    fn run_example_recursive(filename: &str) -> usize {
        let maze = RecursiveMaze::from_file(filename);
        maze.find_shortest_path_len().unwrap()
    }

    #[test]
    fn test_example1() {
        let steps = run_example("./example1.txt");
        assert_eq!(steps, 23);
    }

    #[test]
    fn test_example2() {
        let steps = run_example("./example2.txt");
        assert_eq!(steps, 58);
    }

    #[test]
    fn test_example1_recursive() {
        let steps = run_example_recursive("./example1.txt");
        assert_eq!(steps, 26);
    }

    #[test]
    fn test_example3_recursive() {
        let steps = run_example_recursive("./example3.txt");
        assert_eq!(steps, 396);
    }
}
