mod a_star;
mod grid;
mod maze;
mod path_cache;
mod point;

use a_star::{search, Searchable};
use maze::Maze;
use path_cache::{PathCache, ENTRANCE};

fn main() {
    part1();
    part2();
}

fn part1() {
    let maze = Maze::from_file("./input.txt");
    let paths = PathCache::new(&maze);
    let puzzle = Puzzle::new(&maze, &paths);
    let states = puzzle
        .find_shortest_path()
        .expect("Failed to find shortest path");
    println!("part1: {:?}", states[states.len() - 1].get_steps());
}

fn part2() {
    let maze = Maze::from_file("./correct_input.txt");
    let paths = PathCache::new(&maze);
    let puzzle = Puzzle::new(&maze, &paths);
    let states = puzzle
        .find_shortest_path()
        .expect("Failed to find shortest path");
    println!("part2: {:?}", states[states.len() - 1].get_steps());
}

#[derive(Clone, Debug)]
struct State {
    sorted_keys: Vec<String>,
    sorted_last_keys: Vec<String>,
    last_keys: Vec<String>,
    steps: usize,
}

impl State {
    fn new(keys: Vec<String>, last_keys: Vec<String>, steps: usize) -> Self {
        let mut sorted_keys = keys.clone();
        sorted_keys.sort();

        let mut sorted_last_keys = last_keys.clone();
        sorted_last_keys.sort();

        State {
            sorted_keys,
            sorted_last_keys,
            last_keys,
            steps,
        }
    }

    fn initial_state(n_robots: usize) -> Self {
        let keys = vec![ENTRANCE.to_owned()];
        let last_keys = vec![ENTRANCE.to_owned(); n_robots];
        State::new(keys, last_keys, 0)
    }

    fn final_state(all_keys: Vec<String>) -> Self {
        let mut keys = vec![ENTRANCE.to_owned()];
        keys.extend(all_keys.into_iter());
        keys.sort();
        State::new(keys, vec![], 0) // last_keys and steps don't matter. This is only used for equality
    }

    fn next_state(&self, from_key: &String, next_key: &String, distance: usize) -> Self {
        let mut keys = self.sorted_keys.clone();
        keys.push(next_key.clone());

        let mut last_keys = self.last_keys.clone();
        let index = last_keys
            .iter()
            .position(|key| key == from_key)
            .expect("Failed to get position of previous key");
        last_keys[index] = next_key.clone();

        State::new(keys, last_keys, self.steps + distance)
    }

    fn num_keys(&self) -> usize {
        self.sorted_keys.len() - 1
    }

    fn get_keys(&self) -> &Vec<String> {
        &self.sorted_keys
    }

    fn contains_key(&self, key: &str) -> bool {
        self.sorted_keys.contains(&key.to_owned())
    }

    fn get_last_keys(&self) -> &Vec<String> {
        &self.last_keys
    }

    fn diff_last_keys(&self, other: &State) -> Option<(String, String)> {
        other
            .last_keys
            .iter()
            .position(|key| !self.last_keys.contains(key))
            .map(|index| {
                (
                    self.last_keys[index].clone(),
                    other.last_keys[index].clone(),
                )
            })
    }

    fn get_steps(&self) -> usize {
        self.steps
    }
}

impl std::cmp::Eq for State {}

impl std::cmp::PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.sorted_keys == other.sorted_keys
    }
}

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.sorted_keys.hash(hasher);
        self.sorted_last_keys.hash(hasher);
    }
}

struct Puzzle<'a> {
    paths: &'a PathCache,
    maze: &'a Maze,
}

impl<'a> Puzzle<'a> {
    fn new(maze: &'a Maze, paths: &'a PathCache) -> Self {
        Puzzle { paths, maze }
    }

    fn find_shortest_path(&self) -> Option<Vec<State>> {
        let n_robots = self.maze.get_entrances().len();
        let initial_state = State::initial_state(n_robots);
        let all_keys = self.maze.get_keys();
        let final_state = State::final_state(all_keys);
        search(&initial_state, &final_state, self, |_current, _goal| 0) // djikstra's
    }
}

impl<'a> Searchable<State> for Puzzle<'a> {
    fn distance(&self, current: &State, neighbor: &State) -> usize {
        let (last, next) = current
            .diff_last_keys(neighbor)
            .expect("Failed to get change in last keys");

        let path = self
            .paths
            .get(&last, &next)
            .expect(&format!("Failed to get path from {} to {}", last, next));
        path.len()
    }

    fn get_neighbors(&self, state: &State) -> Vec<State> {
        let last_keys = state.get_last_keys();
        let all_keys = self.maze.get_keys();
        let missing_keys = all_keys
            .into_iter()
            .filter(|key| !state.contains_key(key))
            .collect::<Vec<_>>();

        let mut neighbors = Vec::new();
        for next_key in missing_keys {
            for last_key in last_keys {
                if let Some(path) = self.paths.get(last_key, &next_key) {
                    if path.is_unlocked(state.get_keys()) {
                        let next_state = state.next_state(last_key, &next_key, path.len());
                        neighbors.push(next_state);
                    }
                }
            }
        }
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_example(input: &str) -> (usize, Vec<State>) {
        let maze = Maze::from_string(input);
        let paths = PathCache::new(&maze);
        let puzzle = Puzzle::new(&maze, &paths);
        let states = puzzle
            .find_shortest_path()
            .expect("Failed to find shortest path");
        (states.last().unwrap().get_steps(), states)
    }

    #[test]
    fn test_example1() {
        let input = "#########
                     #b.A.@.a#
                     #########";
        let (steps, _) = run_example(input);
        assert_eq!(steps, 8);
    }

    #[test]
    fn test_example2() {
        let input = "########################
                     #f.D.E.e.C.b.A.@.a.B.c.#
                     ######################.#
                     #d.....................#
                     ########################";
        let (steps, _) = run_example(input);
        assert_eq!(steps, 86);
    }

    #[test]
    fn test_example3() {
        let input = "########################
                     #...............b.C.D.f#
                     #.######################
                     #.....@.a.B.c.d.A.e.F.g#
                     ########################";
        let (steps, _) = run_example(input);
        assert_eq!(steps, 132);
    }

    #[test]
    fn test_example4() {
        let input = "#################
                     #i.G..c...e..H.p#
                     ########.########
                     #j.A..b...f..D.o#
                     ########@########
                     #k.E..a...g..B.n#
                     ########.########
                     #l.F..d...h..C.m#
                     #################";
        let (steps, _) = run_example(input);
        assert_eq!(steps, 136);
    }

    #[test]
    fn test_example5() {
        let input = "########################
                     #@..............ac.GI.b#
                     ###d#e#f################
                     ###A#B#C################
                     ###g#h#i################
                     ########################";
        let (steps, _) = run_example(input);
        assert_eq!(steps, 81);
    }

    #[test]
    fn test_example6() {
        let input = "#######
                     #a.#Cd#
                     ##@#@##
                     #######
                     ##@#@##
                     #cB#.b#
                     #######";
        let (steps, _) = run_example(input);
        assert_eq!(steps, 8);
    }

    #[test]
    fn test_example7() {
        let input = "###############
                     #d.ABC.#.....a#
                     ######@#@######
                     ###############
                     ######@#@######
                     #b.....#.....c#
                     ###############";
        let (steps, _) = run_example(input);
        assert_eq!(steps, 24);
    }

    #[test]
    fn test_example8() {
        let input = "#############
                     #DcBa.#.GhKl#
                     #.###@#@#I###
                     #e#d#####j#k#
                     ###C#@#@###J#
                     #fEbA.#.FgHi#
                     #############";
        let (steps, _) = run_example(input);
        assert_eq!(steps, 32);
    }

    #[test]
    fn test_example9() {
        let input = "#############
                     #g#f.D#..h#l#
                     #F###e#E###.#
                     #dCba@#@BcIJ#
                     #############
                     #nK.L@#@G...#
                     #M###N#H###.#
                     #o#m..#i#jk.#
                     #############";
        let (steps, _) = run_example(input);
        assert_eq!(steps, 72);
    }
}
