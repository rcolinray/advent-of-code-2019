mod lib;

use lib::{is_valid_part1, is_valid_part2, to_digits};

fn main() {
    let num_valid = (235741..706948)
        .filter(|num| {
            let digits = to_digits(*num);
            is_valid_part1(&digits)
        })
        .count();
    println!("part 1: {}", num_valid);

    let num_valid = (235741..706948)
        .filter(|num| {
            let digits = to_digits(*num);
            is_valid_part2(&digits)
        })
        .count();
    println!("part 2: {}", num_valid);
}
