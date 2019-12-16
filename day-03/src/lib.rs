use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops;

pub fn parse_puzzle_input(filename: &str) -> (Wire, Wire) {
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);
  let wires = reader
    .lines()
    .map(|line| parse_wire(&line.unwrap()))
    .collect::<Vec<Wire>>();
  assert_eq!(wires.len(), 2);
  (wires[0].clone(), wires[1].clone())
}

pub fn parse_wire(text: &str) -> Wire {
  text.split(",").map(|s| Point::from_str(s)).collect()
}

pub type Wire = Vec<Point>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl Point {
  pub fn new(x: i32, y: i32) -> Point {
    Point { x, y }
  }

  pub fn from_str(text: &str) -> Point {
    let distance = text.get(1..).map(|t| t.parse::<i32>().unwrap()).unwrap();
    match text.chars().nth(0).unwrap() {
      'L' => Point::new(-distance, 0),
      'R' => Point::new(distance, 0),
      'U' => Point::new(0, distance),
      'D' => Point::new(0, -distance),
      _ => panic!("Could not parse segment '{}'", text),
    }
  }

  pub fn unit(&self) -> Point {
    Point {
      x: self.x.signum(),
      y: self.y.signum(),
    }
  }

  pub fn manhattan_distance(&self) -> i32 {
    self.x.abs() + self.y.abs()
  }

  pub fn between(&self, other: &Point) -> BetweenIter {
    BetweenIter::new(*self, *other)
  }
}

pub struct BetweenIter {
  curr: Point,
  end: Point,
  inc: Point,
}

impl BetweenIter {
  fn new(begin: Point, end: Point) -> BetweenIter {
    let diff = end - begin;
    let inc = diff.unit();
    BetweenIter {
      curr: begin,
      end,
      inc,
    }
  }
}

impl Iterator for BetweenIter {
  type Item = Point;

  fn next(&mut self) -> Option<Point> {
    self.curr += self.inc;
    if self.curr == self.end {
      None
    } else {
      Some(self.curr)
    }
  }
}

impl ops::Add<Point> for Point {
  type Output = Point;

  fn add(self, rhs: Point) -> Point {
    Point {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

impl ops::AddAssign for Point {
  fn add_assign(&mut self, other: Self) {
    *self = Self {
      x: self.x + other.x,
      y: self.y + other.y,
    };
  }
}

impl ops::Sub<Point> for Point {
  type Output = Point;

  fn sub(self, rhs: Point) -> Point {
    Point {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
  }
}

pub fn find_intersections(wire1: &Wire, wire2: &Wire) -> HashMap<Point, usize> {
  let mut map = HashMap::new();
  let mut position = Point::new(0, 0);

  let mut steps = 1;
  for segment in wire1 {
    let new_position = position + *segment;
    for point in position.between(&new_position) {
      map.insert(point, steps);
      steps += 1;
    }
    map.insert(new_position, steps);
    steps += 1;
    position = new_position;
  }

  let mut intersections = HashMap::new();
  let mut position = Point::new(0, 0);
  let mut steps = 1;
  for segment in wire2 {
    let new_position = position + *segment;
    for point in position.between(&new_position) {
      if map.contains_key(&point) {
        let wire1_steps = map.get(&point).unwrap();
        intersections.insert(point, steps + *wire1_steps);
      }
      steps += 1;
    }
    if map.contains_key(&new_position) {
      let wire1_steps = map.get(&new_position).unwrap();
      intersections.insert(new_position, steps + *wire1_steps);
    }
    steps += 1;
    position = new_position;
  }

  intersections
}

pub fn solve_part1(wire1: &Wire, wire2: &Wire) -> Point {
  let intersections = find_intersections(wire1, wire2);
  *intersections
    .keys()
    .min_by(|point1, point2| {
      let d1 = point1.manhattan_distance();
      let d2 = point2.manhattan_distance();
      if d1 < d2 {
        Ordering::Less
      } else if d1 == d2 {
        Ordering::Equal
      } else {
        Ordering::Greater
      }
    })
    .unwrap()
}

pub fn solve_part2(wire1: &Wire, wire2: &Wire) -> usize {
  let intersections = find_intersections(wire1, wire2);
  *intersections.values().min().unwrap()
}

#[cfg(test)]
mod test {
  use super::*;

  use std::collections::hash_map::DefaultHasher;
  use std::hash::Hasher;

  #[test]
  fn test_point_add() {
    let lhs = Point::new(1, 2);
    let rhs = Point::new(3, 4);
    let output = lhs + rhs;
    assert_eq!(output, Point::new(4, 6));
  }

  fn hash_point(p: Point) -> u64 {
    let mut hasher = DefaultHasher::new();
    p.hash(&mut hasher);
    hasher.finish()
  }

  #[test]
  fn test_vec2d_hash() {
    let hash1 = hash_point(Point::new(1, 2));
    let hash2 = hash_point(Point::new(1, 2));
    let hash3 = hash_point(Point::new(3, 4));
    let hash4 = hash_point(Point::new(4, 3));

    assert_eq!(hash1, hash2);
    assert_ne!(hash1, hash3);
    assert_ne!(hash3, hash4);
  }

  #[test]
  fn test_solve1() {
    let wire1 = parse_wire("R8,U5,L5,D3");
    let wire2 = parse_wire("U7,R6,D4,L4");
    let point = solve_part1(&wire1, &wire2);
    assert_eq!(point.manhattan_distance(), 6);
  }

  #[test]
  fn test_solve2() {
    let wire1 = parse_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let wire2 = parse_wire("U62,R66,U55,R34,D71,R55,D58,R83");
    let point = solve_part1(&wire1, &wire2);
    assert_eq!(point.manhattan_distance(), 159);
  }

  #[test]
  fn test_solve3() {
    let wire1 = parse_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    let wire2 = parse_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    let point = solve_part1(&wire1, &wire2);
    assert_eq!(point.manhattan_distance(), 135);
  }

  #[test]
  fn test_solve4() {
    let wire1 = parse_wire("R8,U5,L5,D3");
    let wire2 = parse_wire("U7,R6,D4,L4");
    let steps = solve_part2(&wire1, &wire2);
    assert_eq!(steps, 30);
  }

  #[test]
  fn test_solve5() {
    let wire1 = parse_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let wire2 = parse_wire("U62,R66,U55,R34,D71,R55,D58,R83");
    let steps = solve_part2(&wire1, &wire2);
    assert_eq!(steps, 610);
  }

  #[test]
  fn test_solve6() {
    let wire1 = parse_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    let wire2 = parse_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    let steps = solve_part2(&wire1, &wire2);
    assert_eq!(steps, 410);
  }
}
