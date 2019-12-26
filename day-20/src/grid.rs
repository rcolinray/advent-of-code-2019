use crate::point::Point2D;

pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        let data_len = width * height;
        let data = vec![Default::default(); data_len];
        Grid {
            width,
            height,
            data,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn set(&mut self, point: &Point2D, data: T) {
        let idx = self.point_to_idx(point);
        self.data[idx] = data;
    }

    pub fn get(&self, point: &Point2D) -> &T {
        let idx = self.point_to_idx(point);
        &self.data[idx]
    }

    pub fn get_neighbors(&self, point: &Point2D) -> Vec<Point2D> {
        point.get_neighbors(self.width, self.height)
    }

    fn point_to_idx(&self, point: &Point2D) -> usize {
        (point.y * self.width) + point.x
    }
}
