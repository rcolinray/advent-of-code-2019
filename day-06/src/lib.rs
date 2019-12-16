use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn load_puzzle_input(filename: &str) -> System {
    let file = File::open(filename).unwrap();
    parse_puzzle_input(file)
}

pub fn parse_puzzle_input<T>(buf: T) -> System
where
    T: Read,
{
    let buf_reader = BufReader::new(buf);
    let orbits = buf_reader
        .lines()
        .map(|line| Orbit::from_str(&line.unwrap()))
        .collect::<Vec<_>>();
    System::new(&orbits)
}

#[derive(Debug, PartialEq)]
pub struct Orbit {
    parent: String,
    child: String,
}

impl Orbit {
    fn new(parent: &str, child: &str) -> Self {
        Orbit {
            parent: parent.to_owned(),
            child: child.to_owned(),
        }
    }

    fn from_str(line: &str) -> Self {
        let objects = line.split(")").collect::<Vec<_>>();
        assert_eq!(objects.len(), 2);
        Orbit::new(objects[0], objects[1])
    }
}

const ORIGIN: &str = "COM";

pub struct System {
    orbits: HashMap<String, String>,
}

impl System {
    fn new(orbits: &Vec<Orbit>) -> Self {
        let mut orbit_map = HashMap::new();
        for orbit in orbits.iter() {
            orbit_map.insert(orbit.child.clone(), orbit.parent.clone());
        }
        System { orbits: orbit_map }
    }

    fn size(&self) -> usize {
        self.orbits.len() + 1
    }

    pub fn total_orbit_count(&self) -> usize {
        self.orbits
            .values()
            .fold(0, |total, parent| total + self.count_orbits(parent))
    }

    fn count_orbits(&self, parent: &str) -> usize {
        if parent == ORIGIN {
            1
        } else {
            let next_parent = self.orbits.get(parent).unwrap();
            self.count_orbits(next_parent) + 1
        }
    }

    pub fn count_min_transfers(&self, object1: &str, object2: &str) -> usize {
        let path1 = self.get_path_to_com(object1);
        let path2 = self.get_path_to_com(object2);
        let common_path = path1
            .iter()
            .zip(path2.iter())
            .take_while(|(object1, object2)| object1 == object2)
            .collect::<Vec<_>>();
        path1.len() + path2.len() - 2 * common_path.len()
    }

    fn get_path_to_com(&self, object: &str) -> Vec<&str> {
        let mut curr = object;
        let mut path = Vec::<&str>::new();
        loop {
            let next = self.orbits.get(curr).unwrap();
            path.push(next);
            if next == ORIGIN {
                break;
            }
            curr = next;
        }
        path.reverse();
        path
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_orbit() {
        assert_eq!(Orbit::from_str("AAA)BBB"), Orbit::new("AAA", "BBB"));
    }

    #[test]
    fn test_parse_puzzle_input1() {
        let input = "COM)AAA";
        let system = parse_puzzle_input(input.as_bytes());
        assert_eq!(system.size(), 2);
    }

    #[test]
    fn test_parse_puzzle_input2() {
        let input = "COM)AAA\nAAA)BBB";
        let system = parse_puzzle_input(input.as_bytes());
        assert_eq!(system.size(), 3);
    }

    #[test]
    fn test_count_orbits() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        let system = parse_puzzle_input(input.as_bytes());
        assert_eq!(system.size(), 12);
        assert_eq!(system.total_orbit_count(), 42);
    }

    #[test]
    fn test_count_min_transfers() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
        let system = parse_puzzle_input(input.as_bytes());
        assert_eq!(system.size(), 14);
        assert_eq!(system.count_min_transfers("YOU", "SAN"), 4);
    }
}
