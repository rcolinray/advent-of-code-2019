use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Read};

use crate::reaction::Reaction;

pub fn load_puzzle_input(filename: &str) -> Vec<Reaction> {
    let file = File::open(filename).expect(&format!("Failed to open {}", filename));
    parse_puzzle_input(file)
}

pub fn parse_puzzle_input<R: Read>(buf: R) -> Vec<Reaction> {
    let reader = BufReader::new(buf);
    reader
        .lines()
        .map(|line| {
            let string = line.expect("Failed to read line from puzzle input");
            Reaction::from_string(&string).expect(&format!("Failed to parse Reaction: {}", string))
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_puzzle_input() {
        let input = "10 ORE => 10 A
                     1 ORE => 1 B
                     7 A, 1 B => 1 C
                     7 A, 1 C => 1 D
                     7 A, 1 D => 1 E
                     7 A, 1 E => 1 FUEL";
        let reactions = parse_puzzle_input(input.as_bytes());
        assert_eq!(reactions.len(), 6);
    }
}
