pub struct Digits {
    n: Option<i32>,
}

impl Digits {
    fn new(n: i32) -> Self {
        Digits { n: Some(n) }
    }
}

impl Iterator for Digits {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.n {
            None => None,
            Some(n) => {
                if n < 10 {
                    let next = Some(n);
                    self.n = None;
                    next
                } else {
                    let next = n % 10;
                    self.n = Some(n / 10);
                    Some(next)
                }
            }
        }
    }
}

pub fn to_digits(n: i32) -> Digits {
    Digits::new(n)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(to_digits(0).collect::<Vec<_>>(), vec![0]);
        assert_eq!(to_digits(1).collect::<Vec<_>>(), vec![1]);
        assert_eq!(to_digits(10).collect::<Vec<_>>(), vec![0, 1]);
        assert_eq!(to_digits(42).collect::<Vec<_>>(), vec![2, 4]);
        assert_eq!(to_digits(100).collect::<Vec<_>>(), vec![0, 0, 1]);
    }
}
