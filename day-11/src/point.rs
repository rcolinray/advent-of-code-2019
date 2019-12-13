#[derive(Default, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn at(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point::at(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point::at(self.x - rhs.x, self.y - rhs.y)
    }
}
