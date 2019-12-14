use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type MatData = [[i32; 3]; 4];

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Mat4x3 {
    data: MatData,
}

impl Mat4x3 {
    pub fn new(data: MatData) -> Self {
        Mat4x3 { data }
    }

    pub fn from_file(filename: &str) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<_>>();
        assert_eq!(lines.len(), 4);
        let io = Mat4x3::parse_row(&lines[0]);
        let europa = Mat4x3::parse_row(&lines[1]);
        let ganymede = Mat4x3::parse_row(&lines[2]);
        let callisto = Mat4x3::parse_row(&lines[3]);
        Mat4x3::new([io, europa, ganymede, callisto])
    }

    pub fn from_cols(x: [i32; 4], y: [i32; 4], z: [i32; 4]) -> Self {
        Mat4x3 {
            data: [
                [x[0], y[0], z[0]],
                [x[1], y[1], z[1]],
                [x[2], y[2], z[2]],
                [x[3], y[3], z[3]],
            ],
        }
    }

    fn parse_row(string: &str) -> [i32; 3] {
        let components = string
            .trim_start_matches('<')
            .trim_end_matches('>')
            .split(", ")
            .collect::<Vec<_>>();
        assert_eq!(components.len(), 3);
        let x = components[0]
            .trim_start_matches("x=")
            .parse::<i32>()
            .unwrap();
        let y = components[1]
            .trim_start_matches("y=")
            .parse::<i32>()
            .unwrap();
        let z = components[2]
            .trim_start_matches("z=")
            .parse::<i32>()
            .unwrap();
        [x, y, z]
    }

    #[inline(always)]
    pub fn sum_rows(&self) -> [i32; 4] {
        [
            self.data[0][0].abs() + self.data[0][1].abs() + self.data[0][2].abs(),
            self.data[1][0].abs() + self.data[1][1].abs() + self.data[1][2].abs(),
            self.data[2][0].abs() + self.data[2][1].abs() + self.data[2][2].abs(),
            self.data[3][0].abs() + self.data[3][1].abs() + self.data[3][2].abs(),
        ]
    }

    #[inline(always)]
    pub fn axis(&self, i: usize) -> [i32; 4] {
        [
            self.data[0][i],
            self.data[1][i],
            self.data[2][i],
            self.data[3][i],
        ]
    }
}

impl std::fmt::Display for Mat4x3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:5}, {:5}, {:5}\n {:5}, {:5}, {:5}\n {:5}, {:5}, {:5}\n {:5}, {:5}, {:5}   ]",
            self.data[0][0],
            self.data[0][1],
            self.data[0][2],
            self.data[1][0],
            self.data[1][1],
            self.data[1][2],
            self.data[2][0],
            self.data[2][1],
            self.data[2][2],
            self.data[3][0],
            self.data[3][1],
            self.data[3][2]
        )
    }
}

impl std::ops::AddAssign<Mat4x3> for Mat4x3 {
    fn add_assign(&mut self, rhs: Mat4x3) {
        self.data[0][0] += rhs.data[0][0];
        self.data[0][1] += rhs.data[0][1];
        self.data[0][2] += rhs.data[0][2];
        self.data[1][0] += rhs.data[1][0];
        self.data[1][1] += rhs.data[1][1];
        self.data[1][2] += rhs.data[1][2];
        self.data[2][0] += rhs.data[2][0];
        self.data[2][1] += rhs.data[2][1];
        self.data[2][2] += rhs.data[2][2];
        self.data[3][0] += rhs.data[3][0];
        self.data[3][1] += rhs.data[3][1];
        self.data[3][2] += rhs.data[3][2];
    }
}
