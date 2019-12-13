use core::f32::EPSILON;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let map = load_map("./input.txt");
    let (best_point, num_sight_lines) = find_best_asteroid(&map).unwrap();
    let mut station = Station::new(best_point, &map);
    let vaporized = station.seek_and_destroy();
    let result = vaporized[199];
    println!("part 2: {:?}", result);
}

fn find_best_asteroid(map: &AsteroidMap) -> Option<(Point, usize)> {
    let mut best_point: Option<Point> = None;
    let mut max_sight_lines = 0;
    for point in map.iter() {
        let num_sight_lines = count_sight_lines(*point, &map);
        if num_sight_lines > max_sight_lines {
            best_point = Some(*point);
            max_sight_lines = num_sight_lines;
        }
    }

    best_point.map(|point| (point, max_sight_lines))
}

fn count_sight_lines(a: Point, map: &AsteroidMap) -> usize {
    map.iter()
        .filter(|&point| *point != a && has_line_of_sight(a, *point, map))
        .count()
}

fn has_line_of_sight(a: Point, b: Point, map: &AsteroidMap) -> bool {
    a.raycast_to(b).all(|point| !map.contains(&point))
}

fn load_map(filename: &str) -> AsteroidMap {
    let file = File::open(filename).unwrap();
    parse_map(file)
}

#[allow(dead_code)]
fn map_from_string(string: &str) -> AsteroidMap {
    parse_map(string.as_bytes())
}

fn parse_map<T>(buf: T) -> AsteroidMap
where
    T: Read,
{
    let reader = BufReader::new(buf);
    let mut x = 0;
    let mut y = 0;
    let mut asteroids = HashSet::new();
    for line in reader.lines() {
        for c in line.unwrap().trim_start().chars() {
            if c == '#' {
                asteroids.insert(Point::at(x, y));
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }
    asteroids
}

type AsteroidMap = HashSet<Point>;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn at(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn scale(self, magnitude: i32) -> Point {
        Point::at(self.x / magnitude, self.y / magnitude)
    }

    fn raycast_to(self, target: Point) -> RayCast {
        RayCast::new(self, target)
    }

    fn magnitude(&self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }

    fn dot(self, rhs: Point) -> f32 {
        ((self.x * rhs.x) + (self.y * rhs.y)) as f32
    }

    fn cross(self, rhs: Point) -> f32 {
        ((self.x * rhs.y) - (self.y * rhs.x)) as f32
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point::at(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Self::Output {
        Point::at(self.x - other.x, self.y - other.y)
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

struct RayCast {
    curr: Point,
    end: Point,
    step: Point,
}

impl RayCast {
    fn new(begin: Point, end: Point) -> Self {
        let diff = end - begin;
        let div = gcd(diff.x, diff.y).abs();
        let step = diff.scale(div);
        RayCast {
            curr: begin,
            end: end,
            step,
        }
    }
}

impl Iterator for RayCast {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.curr + self.step;
        if next == self.end {
            None
        } else {
            self.curr = next;
            Some(next)
        }
    }
}

struct Station {
    point: Point,
    aim: Point,
    map: AsteroidMap,
    vaporized: Vec<Point>,
}

impl Station {
    fn new(point: Point, map: &AsteroidMap) -> Self {
        let mut new_map = map.clone();
        new_map.remove(&point);
        Station {
            point,
            aim: point + Point::at(0, -1),
            map: new_map,
            vaporized: Vec::new(),
        }
    }

    fn seek_and_destroy(&mut self) -> Vec<Point> {
        while self.map.len() > 0 {
            if let Some((asteroid, _)) = self.seek() {
                self.map.remove(&asteroid);
                self.vaporized.push(asteroid);
                self.aim = asteroid;
            } else {
                break;
            }
        }
        self.vaporized.clone()
    }

    fn seek(&self) -> Option<(Point, f32)> {
        self.map
            .iter()
            .filter(|&point| has_line_of_sight(self.point, *point, &self.map))
            .fold(None, |acc, &point| {
                let angle = calc_angle(self.point, self.aim, point);
                if self.vaporized.len() > 0 && self.map.len() > 1 && angle < EPSILON {
                    acc
                } else if let Some((_, closest_angle)) = acc {
                    if angle < closest_angle {
                        Some((point, angle))
                    } else {
                        acc
                    }
                } else {
                    Some((point, angle))
                }
            })
    }
}

fn calc_angle(origin: Point, a: Point, b: Point) -> f32 {
    let u = a - origin;
    let v = b - origin;
    let angle = (u.dot(v) / (u.magnitude() * v.magnitude()))
        .acos()
        .to_degrees();
    if u.cross(v) < 0.0 {
        // Right-hand rule!
        360.0 - angle
    } else {
        angle
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_magnitude() {
        let point = Point::at(3, 4);
        assert_eq!(point.magnitude(), 5.0);
    }

    #[test]
    fn test_dot() {
        let dot = Point::at(2, 2).dot(Point::at(0, 3));
        assert_eq!(dot, 6.0);
    }

    #[test]
    fn test_cross() {
        let cross = Point::at(0, 1).cross(Point::at(1, 0));
        assert_eq!(cross, -1.0);

        let cross = Point::at(0, 1).cross(Point::at(0, -1));
        assert_eq!(cross, 0.0);

        let cross = Point::at(0, 1).cross(Point::at(-1, 0));
        assert_eq!(cross, 1.0);
    }

    #[test]
    fn test_angle1() {
        let origin = Point::at(5, 5);

        let angle = calc_angle(origin, Point::at(5, 4), Point::at(5, 4));
        assert_eq!(angle.round(), 0.0);

        let angle = calc_angle(origin, Point::at(5, 4), Point::at(6, 4));
        assert_eq!(angle.round(), 45.0);

        let angle = calc_angle(origin, Point::at(5, 4), Point::at(6, 5));
        assert_eq!(angle.round(), 90.0);

        let angle = calc_angle(origin, Point::at(5, 4), Point::at(6, 6));
        assert_eq!(angle.round(), 135.0);

        let angle = calc_angle(origin, Point::at(5, 4), Point::at(5, 6));
        assert_eq!(angle.round(), 180.0);

        let angle = calc_angle(origin, Point::at(5, 4), Point::at(4, 6));
        assert_eq!(angle.round(), 225.0);

        let angle = calc_angle(origin, Point::at(5, 4), Point::at(4, 5));
        assert_eq!(angle.round(), 270.0);

        let angle = calc_angle(origin, Point::at(5, 4), Point::at(4, 4));
        assert_eq!(angle.round(), 315.0);
    }

    #[test]
    fn test_angle2() {
        let origin = Point::at(5, 5);

        let angle = calc_angle(origin, Point::at(5, 6), Point::at(5, 6));
        assert_eq!(angle.round(), 0.0);

        let angle = calc_angle(origin, Point::at(5, 6), Point::at(4, 6));
        assert_eq!(angle.round(), 45.0);

        let angle = calc_angle(origin, Point::at(5, 6), Point::at(4, 5));
        assert_eq!(angle.round(), 90.0);

        let angle = calc_angle(origin, Point::at(5, 6), Point::at(4, 4));
        assert_eq!(angle.round(), 135.0);

        let angle = calc_angle(origin, Point::at(5, 6), Point::at(5, 4));
        assert_eq!(angle.round(), 180.0);

        let angle = calc_angle(origin, Point::at(5, 6), Point::at(6, 4));
        assert_eq!(angle.round(), 225.0);

        let angle = calc_angle(origin, Point::at(5, 6), Point::at(6, 5));
        assert_eq!(angle.round(), 270.0);

        let angle = calc_angle(origin, Point::at(5, 6), Point::at(6, 6));
        assert_eq!(angle.round(), 315.0);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(3, 4), 1);
        assert_eq!(gcd(4, 3), 1);
        assert_eq!(gcd(2, 2), 2);
        assert_eq!(gcd(0, 2), 2);
        assert_eq!(gcd(2, 0), 2);
        assert_eq!(gcd(9, 6), 3);
        assert_eq!(gcd(4, 6), 2);
        assert_eq!(gcd(6, 4), 2);
    }

    #[test]
    fn test_raycast_one_step() {
        let ray = RayCast::new(Point::at(0, 0), Point::at(2, 2));
        let points = ray.collect::<Vec<_>>();
        assert_eq!(points.len(), 1);
        assert_eq!(points[0], Point::at(1, 1));
    }

    #[test]
    fn test_raycast_no_steps() {
        let ray = RayCast::new(Point::at(0, 0), Point::at(3, 4));
        let points = ray.collect::<Vec<_>>();
        assert_eq!(points.len(), 0);
    }

    #[test]
    fn test_raycast_two_steps() {
        let ray = RayCast::new(Point::at(0, 0), Point::at(9, 6));
        let points = ray.collect::<Vec<_>>();
        assert_eq!(points.len(), 2);
        assert_eq!(points, vec![Point::at(3, 2), Point::at(6, 4)]);
    }

    #[test]
    fn test_raycast_negative_step() {
        let ray = RayCast::new(Point::at(2, 2), Point::at(0, 0));
        let points = ray.collect::<Vec<_>>();
        assert_eq!(points.len(), 1);
        assert_eq!(points[0], Point::at(1, 1));
    }

    #[test]
    fn test_raycast_one_direction() {
        let ray = RayCast::new(Point::at(0, 0), Point::at(0, 2));
        let points = ray.collect::<Vec<_>>();
        assert_eq!(points.len(), 1);
        assert_eq!(points[0], Point::at(0, 1));
    }

    fn expect_map(string: &str, point: Point, sight_lines: usize) {
        let map = map_from_string(string);
        let result = find_best_asteroid(&map);
        assert_eq!(result, Some((point, sight_lines)));
    }

    #[test]
    fn test_map1() {
        let map = ".#..#
                   .....
                   #####
                   ....#
                   ...##";
        expect_map(&map, Point::at(3, 4), 8);
    }

    #[test]
    fn test_map2() {
        let map = "......#.#.
                   #..#.#....
                   ..#######.
                   .#.#.###..
                   .#..#.....
                   ..#....#.#
                   #..#....#.
                   .##.#..###
                   ##...#..#.
                   .#....####";
        expect_map(map, Point::at(5, 8), 33);
    }

    #[test]
    fn test_map3() {
        let map = "#.#...#.#.
                   .###....#.
                   .#....#...
                   ##.#.#.#.#
                   ....#.#.#.
                   .##..###.#
                   ..#...##..
                   ..##....##
                   ......#...
                   .####.###.";
        expect_map(map, Point::at(1, 2), 35);
    }

    #[test]
    fn test_map4() {
        let map = ".#..#..###
                   ####.###.#
                   ....###.#.
                   ..###.##.#
                   ##.##.#.#.
                   ....###..#
                   ..#.#..#.#
                   #..#.#.###
                   .##...##.#
                   .....#.#..";
        expect_map(map, Point::at(6, 3), 41);
    }

    #[test]
    fn test_map5() {
        let map = ".#..##.###...#######
                   ##.############..##.
                   .#.######.########.#
                   .###.#######.####.#.
                   #####.##.#.##.###.##
                   ..#####..#.#########
                   ####################
                   #.####....###.#.#.##
                   ##.#################
                   #####.##.###..####..
                   ..######..##.#######
                   ####.##.####...##..#
                   .#####..#.######.###
                   ##...#.##########...
                   #.##########.#######
                   .####.#.###.###.#.##
                   ....##.##.###..#####
                   .#.#.###########.###
                   #.#.#.#####.####.###
                   ###.##.####.##.#..##";
        expect_map(map, Point::at(11, 13), 210);
    }

    #[test]
    fn test_seek_and_destroy1() {
        let string = ".#..#
                      .....
                      #####
                      ....#
                      ...##";
        let map = map_from_string(string);

        let (best_point, _) = find_best_asteroid(&map).unwrap();
        println!("{:?}", best_point);

        let mut station = Station::new(best_point, &map);
        let vaporized = station.seek_and_destroy();
        assert_eq!(
            vaporized,
            vec![
                Point::at(3, 2),
                Point::at(4, 0),
                Point::at(4, 2),
                Point::at(4, 3),
                Point::at(4, 4),
                Point::at(0, 2),
                Point::at(1, 2),
                Point::at(2, 2),
                Point::at(1, 0)
            ]
        );
    }

    #[test]
    fn test_seek_and_destroy2() {
        let string = ".#....#####...#..
                      ##...##.#####..##
                      ##...#...#.#####.
                      ..#.....#...###..
                      ..#.#.....#....##";
        let map = map_from_string(string);

        let (best_point, _) = find_best_asteroid(&map).unwrap();

        let mut station = Station::new(best_point, &map);
        let vaporized = station.seek_and_destroy();

        assert_eq!(
            vaporized,
            vec![
                Point { x: 8, y: 1 },
                Point { x: 9, y: 0 },
                Point { x: 9, y: 1 },
                Point { x: 10, y: 0 },
                Point { x: 9, y: 2 },
                Point { x: 11, y: 1 },
                Point { x: 12, y: 1 },
                Point { x: 11, y: 2 },
                Point { x: 15, y: 1 },
                Point { x: 12, y: 2 },
                Point { x: 13, y: 2 },
                Point { x: 14, y: 2 },
                Point { x: 15, y: 2 },
                Point { x: 12, y: 3 },
                Point { x: 16, y: 4 },
                Point { x: 15, y: 4 },
                Point { x: 10, y: 4 },
                Point { x: 4, y: 4 },
                Point { x: 2, y: 4 },
                Point { x: 2, y: 3 },
                Point { x: 0, y: 2 },
                Point { x: 1, y: 2 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 5, y: 2 },
                Point { x: 1, y: 0 },
                Point { x: 5, y: 1 },
                Point { x: 6, y: 1 },
                Point { x: 6, y: 0 },
                Point { x: 7, y: 0 },
                Point { x: 8, y: 0 },
                Point { x: 10, y: 1 },
                Point { x: 14, y: 0 },
                Point { x: 16, y: 1 },
                Point { x: 13, y: 3 },
                Point { x: 14, y: 3 }
            ]
        );
    }

    #[test]
    fn test_seek_and_destroy3() {
        let string = ".#..##.###...#######
                      ##.############..##.
                      .#.######.########.#
                      .###.#######.####.#.
                      #####.##.#.##.###.##
                      ..#####..#.#########
                      ####################
                      #.####....###.#.#.##
                      ##.#################
                      #####.##.###..####..
                      ..######..##.#######
                      ####.##.####...##..#
                      .#####..#.######.###
                      ##...#.##########...
                      #.##########.#######
                      .####.#.###.###.#.##
                      ....##.##.###..#####
                      .#.#.###########.###
                      #.#.#.#####.####.###
                      ###.##.####.##.#..##";
        let map = map_from_string(string);

        let (best_point, _) = find_best_asteroid(&map).unwrap();

        let mut station = Station::new(best_point, &map);
        let vaporized = station.seek_and_destroy();
        assert_eq!(vaporized[0], Point::at(11, 12));
        assert_eq!(vaporized[1], Point::at(12, 1));
        assert_eq!(vaporized[2], Point::at(12, 2));
        assert_eq!(vaporized[9], Point::at(12, 8));
        assert_eq!(vaporized[19], Point::at(16, 0));
        assert_eq!(vaporized[49], Point::at(16, 9));
        assert_eq!(vaporized[99], Point::at(10, 16));
        assert_eq!(vaporized[198], Point::at(9, 6));
        assert_eq!(vaporized[199], Point::at(8, 2));
        assert_eq!(vaporized[200], Point::at(10, 9));
        assert_eq!(vaporized[298], Point::at(11, 1));
    }
}
