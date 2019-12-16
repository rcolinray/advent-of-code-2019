pub type Digits = Vec<u32>;

pub fn to_digits(num: u32) -> Digits {
  num
    .to_string()
    .chars()
    .map(|c| {
      assert_eq!(c.is_digit(10), true);
      c.to_digit(10).unwrap()
    })
    .collect()
}

pub fn is_valid_part1(digits: &Digits) -> bool {
  digits.len() == 6 && has_two_same_adjacent(digits) && are_digits_increasing(digits)
}

fn are_digits_increasing(digits: &Digits) -> bool {
  let mut prev = &digits[0];
  digits[1..].iter().all(|next| {
    let greater = next >= prev;
    prev = next;
    greater
  })
}

fn has_two_same_adjacent(digits: &Digits) -> bool {
  let mut prev = &digits[0];
  digits[1..].iter().any(|next| {
    let equal = next == prev;
    prev = next;
    equal
  })
}

fn digit_groups(digits: &Digits) -> Vec<Digits> {
  let mut prev = &digits[0];
  let mut groups = Vec::new();
  let mut group = vec![*prev];

  for next in digits[1..].iter() {
    if next == prev {
      group.push(*next);
    } else {
      groups.push(group);
      group = vec![*next];
    }

    prev = next;
  }
  groups.push(group);

  groups
}

fn has_groups_of_two(digits: &Digits) -> bool {
  let groups = digit_groups(digits);
  groups.iter().any(|group| group.len() == 2)
}

pub fn is_valid_part2(digits: &Digits) -> bool {
  is_valid_part1(digits) && has_groups_of_two(digits)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_to_digits() {
    assert_eq!(to_digits(111111), vec![1, 1, 1, 1, 1, 1]);
    assert_eq!(to_digits(223450), vec![2, 2, 3, 4, 5, 0]);
    assert_eq!(to_digits(123789), vec![1, 2, 3, 7, 8, 9]);
  }

  #[test]
  fn test_increasing() {
    assert_eq!(are_digits_increasing(&to_digits(111111)), true);
    assert_eq!(are_digits_increasing(&to_digits(223450)), false);
    assert_eq!(are_digits_increasing(&to_digits(123789)), true);
  }

  #[test]
  fn test_same_adjacent() {
    assert_eq!(has_two_same_adjacent(&to_digits(111111)), true);
    assert_eq!(has_two_same_adjacent(&to_digits(223450)), true);
    assert_eq!(has_two_same_adjacent(&to_digits(123789)), false);
  }

  #[test]
  fn test_valid_part1() {
    assert_eq!(is_valid_part1(&to_digits(111111)), true);
    assert_eq!(is_valid_part1(&to_digits(223450)), false);
    assert_eq!(is_valid_part1(&to_digits(123789)), false);
  }

  #[test]
  fn test_digit_groups() {
    assert_eq!(
      digit_groups(&to_digits(111111)),
      vec![vec![1, 1, 1, 1, 1, 1]]
    );
    assert_eq!(
      digit_groups(&to_digits(112233)),
      vec![vec![1, 1], vec![2, 2], vec![3, 3]]
    );
    assert_eq!(
      digit_groups(&to_digits(123444)),
      vec![vec![1], vec![2], vec![3], vec![4, 4, 4]]
    );
    assert_eq!(
      digit_groups(&to_digits(111122)),
      vec![vec![1, 1, 1, 1], vec![2, 2]]
    );
    assert_eq!(
      digit_groups(&to_digits(123789)),
      vec![vec![1], vec![2], vec![3], vec![7], vec![8], vec![9]]
    );
  }

  #[test]
  fn test_has_groups_of_two() {
    assert_eq!(has_groups_of_two(&to_digits(111111)), false);
    assert_eq!(has_groups_of_two(&to_digits(112233)), true);
    assert_eq!(has_groups_of_two(&to_digits(123444)), false);
    assert_eq!(has_groups_of_two(&to_digits(111122)), true);
    assert_eq!(has_groups_of_two(&to_digits(123789)), false);
  }

  #[test]
  fn test_is_valid_part2() {
    assert_eq!(is_valid_part2(&to_digits(111111)), false);
    assert_eq!(is_valid_part2(&to_digits(223450)), false);
    assert_eq!(is_valid_part2(&to_digits(123789)), false);
    assert_eq!(is_valid_part2(&to_digits(112233)), true);
    assert_eq!(is_valid_part2(&to_digits(123444)), false);
    assert_eq!(is_valid_part2(&to_digits(111122)), true);
  }
}
