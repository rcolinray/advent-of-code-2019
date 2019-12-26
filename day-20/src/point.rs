#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point2D {
    pub x: usize,
    pub y: usize,
}

impl Point2D {
    pub fn new(x: usize, y: usize) -> Self {
        Point2D { x, y }
    }

    pub fn get_neighbors(&self, width: usize, height: usize) -> Vec<Point2D> {
        let mut neighbors = Vec::new();
        if self.x < width - 1 {
            neighbors.push(*self + Point2D::new(1, 0));
        }
        if self.y < height - 1 {
            neighbors.push(*self + Point2D::new(0, 1));
        }
        if self.x > 0 {
            neighbors.push(*self - Point2D::new(1, 0));
        }
        if self.y > 0 {
            neighbors.push(*self - Point2D::new(0, 1));
        }
        neighbors
    }
}

impl std::ops::Add<Point2D> for Point2D {
    type Output = Point2D;

    fn add(self, rhs: Point2D) -> Point2D {
        Point2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub<Point2D> for Point2D {
    type Output = Point2D;

    fn sub(self, rhs: Point2D) -> Point2D {
        Point2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        let point1 = Point2D::new(0, 0);
        let point2 = Point2D::new(1, 0);
        let point3 = Point2D::new(0, 1);
        let point4 = Point2D::new(1, 1);

        assert_eq!(point1 < point2, true);
        assert_eq!(point1 < point3, true);
        assert_eq!(point1 < point4, true);

        assert_eq!(point2 < point4, true);
        assert_eq!(point3 < point4, true);
    }
}
