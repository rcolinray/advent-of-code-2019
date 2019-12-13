use crate::hull::Hull;
use crate::intcode::IntcodeComputer;
use crate::point::Point;

pub const TURN_LEFT: i64 = 0;
pub const TURN_RIGHT: i64 = 1;

fn rotate(facing: Point, dir: i64) -> Point {
    let angle: f32 = match dir {
        TURN_LEFT => 90.0,
        TURN_RIGHT => -90.0,
        _ => panic!("Unrecognized dir {}", dir),
    };

    let x = facing.x as f32;
    let y = facing.y as f32;

    let new_x = (x * angle.cos()) - (y * angle.sin());
    let new_y = (x * angle.sin()) + (y * angle.cos());

    Point::at(new_x.round() as i32, new_y.round() as i32)
}

pub struct Robot<T>
where
    T: IntcodeComputer,
{
    cpu: T,
    point: Point,
    facing: Point,
}

impl<T: IntcodeComputer> Robot<T> {
    pub fn new(cpu: T) -> Self {
        Robot {
            cpu,
            point: Point::at(0, 0),
            facing: Point::at(0, 1),
        }
    }

    pub fn get_location(&self) -> Point {
        self.point
    }

    pub fn get_direction(&self) -> Point {
        self.facing
    }

    pub fn start(&mut self) {
        self.cpu.start();
    }

    pub fn is_running(&self) -> bool {
        self.cpu.is_running()
    }

    pub fn scan(&mut self, hull: &Hull) {
        let color = hull.get_color(&self.point);
        self.cpu.send_input(color);
    }

    pub fn paint(&mut self, hull: &mut Hull) {
        if let Some((color, dir)) = self.read_output_pair() {
            // println!("paint {}, move {}", color, dir);
            hull.paint(self.point, color);
            self.turn(dir);
            self.move_forward();
            // println!("at {:?}, facing {:?}", self.point, self.facing);
        }
    }
    fn turn(&mut self, dir: i64) {
        self.facing = rotate(self.facing, dir);
    }

    fn move_forward(&mut self) {
        self.point = self.point + self.facing;
    }

    fn read_output_pair(&mut self) -> Option<(i64, i64)> {
        let color = self.cpu.recv_output();
        let dir = self.cpu.recv_output();
        match (color, dir) {
            (Some(color), Some(dir)) => Some((color, dir)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate_right() {
        assert_eq!(rotate(Point::at(0, 1), TURN_RIGHT), Point::at(1, 0));
        assert_eq!(rotate(Point::at(1, 0), TURN_RIGHT), Point::at(0, -1));
        assert_eq!(rotate(Point::at(0, -1), TURN_RIGHT), Point::at(-1, 0));
        assert_eq!(rotate(Point::at(-1, 0), TURN_RIGHT), Point::at(0, 1));
    }

    #[test]
    fn test_rotate_left() {
        assert_eq!(rotate(Point::at(0, 1), TURN_LEFT), Point::at(-1, 0));
        assert_eq!(rotate(Point::at(-1, 0), TURN_LEFT), Point::at(0, -1));
        assert_eq!(rotate(Point::at(0, -1), TURN_LEFT), Point::at(1, 0));
        assert_eq!(rotate(Point::at(1, 0), TURN_LEFT), Point::at(0, 1));
    }
}
