#[derive(Default, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector { x, y }
    }
}

impl std::ops::Add<Vector<i64>> for Vector<i64> {
    type Output = Vector<i64>;

    fn add(self, rhs: Vector<i64>) -> Vector<i64> {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::AddAssign<Vector<i64>> for Vector<i64> {
    fn add_assign(&mut self, rhs: Vector<i64>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub<Vector<i64>> for Vector<i64> {
    type Output = Vector<i64>;

    fn sub(self, rhs: Vector<i64>) -> Vector<i64> {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}
