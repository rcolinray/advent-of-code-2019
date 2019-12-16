mod fft;
mod worker;

use fft::fft;
use worker::Worker;

use std::fs;
use std::io::prelude::*;
use std::io::{self, Write};
use std::time::SystemTime;

macro_rules! bench {
    ($desc:literal, $op:stmt) => {
        let now = SystemTime::now();
        print!("{}...", $desc);
        io::stdout().flush().unwrap();
        $op
        println!(" {}ms", now.elapsed().unwrap().as_millis());
    };
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = load_puzzle_input("./input.txt");
    let output = run_fft_phases(input, 100);
    println!("part 1: {:?}", &output[..8]);
}

fn part2() {
    let input = load_puzzle_input("./input.txt");
    let offset = offset_to_index(&input[..7]);
    println!("offset: {}", offset);
    let real_len = input.len() * 10_000;
    println!("real input length: {}", real_len);
    assert!(offset > real_len / 2);
    println!("optimized length: {}", real_len - offset);
    let real_input = input
        .iter()
        .cycle()
        .skip(offset)
        .take(real_len - offset)
        .copied()
        .collect::<Vec<_>>();

    let mut rev_input = real_input.iter().rev().copied().collect::<Vec<_>>();

    for _ in 0..100 {
        let mut partial_sum = 0;
        let mut output = Vec::new();
        for x in rev_input.iter() {
            partial_sum += x;
            output.push((partial_sum % 10).abs());
        }
        rev_input = output;
    }

    let final_output = rev_input.iter().rev().copied().collect::<Vec<_>>();
    println!("message: {:?}", &final_output[0..8]);
}

fn offset_to_index(offset: &[i32]) -> usize {
    offset
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &d)| acc + (10 as i32).pow(i as u32) * d) as usize
}

fn load_puzzle_input(filename: &str) -> Vec<i32> {
    let mut file = fs::File::open(filename).expect("Failed to open puzzle input");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read puzzle input");
    parse_puzzle_input(&contents)
}

fn parse_puzzle_input(contents: &str) -> Vec<i32> {
    contents
        .chars()
        .map(|c| c.to_digit(10).expect("Failed to parse digit") as i32)
        .collect::<Vec<_>>()
}

fn run_fft_phases(data: Vec<i32>, n: usize) -> Vec<i32> {
    (0..n).fold(data, |prev, _| calc_fft_phase(&prev))
}

fn calc_fft_phase(data: &Vec<i32>) -> Vec<i32> {
    (0..data.len())
        .map(|index| fft(0, index, data))
        .collect::<Vec<_>>()
}

const NUM_WORKERS: usize = 8;

fn run_fft_phases_parallel(offset: usize, data: Vec<i32>, n: usize) -> Vec<i32> {
    (0..n).fold(data, |prev, i| {
        let now = SystemTime::now();
        let result = calc_fft_phase_parallel(offset, &prev);
        println!("iteration {} took {}s", i, now.elapsed().unwrap().as_secs());
        result
    })
}

fn calc_fft_phase_parallel(offset: usize, data: &Vec<i32>) -> Vec<i32> {
    let n = data.len();
    let mut block_size = n / NUM_WORKERS;
    if block_size * NUM_WORKERS < n {
        block_size += 1;
    }

    let workers = (0..n)
        .step_by(block_size)
        .map(|i| {
            let last_index = n.min(i + block_size);
            let indices = (i..last_index).collect::<Vec<_>>();
            Worker::start(offset, &indices, data)
        })
        .collect::<Vec<_>>();

    workers
        .iter()
        .flat_map(|worker| {
            let results = worker.get_results();
            results
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_fft_phase() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let phase1 = calc_fft_phase(&input);
        assert_eq!(phase1, vec![4, 8, 2, 2, 6, 1, 5, 8]);
        let phase2 = calc_fft_phase(&phase1);
        assert_eq!(phase2, vec![3, 4, 0, 4, 0, 4, 3, 8]);
        let phase3 = calc_fft_phase(&phase2);
        assert_eq!(phase3, vec![0, 3, 4, 1, 5, 5, 1, 8]);
        let phase4 = calc_fft_phase(&phase3);
        assert_eq!(phase4, vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn test_run_fft_phases() {
        let input = parse_puzzle_input("80871224585914546619083218645595");
        let output = run_fft_phases(input, 100);
        assert_eq!(&output[..8], &[2, 4, 1, 7, 6, 1, 7, 6]);

        let input = parse_puzzle_input("19617804207202209144916044189917");
        let output = run_fft_phases(input, 100);
        assert_eq!(&output[..8], &[7, 3, 7, 4, 5, 4, 1, 8]);

        let input = parse_puzzle_input("69317163492948606335995924319873");
        let output = run_fft_phases(input, 100);
        assert_eq!(&output[..8], &[5, 2, 4, 3, 2, 1, 3, 3]);
    }

    #[test]
    fn test_offset_to_index() {
        let offset = &[1, 2, 3, 4, 5, 6, 7];
        assert_eq!(offset_to_index(offset), 1234567);
    }

    #[test]
    fn test_offset_optimization() {
        let input = parse_puzzle_input("80871224585914546619083218645595");
        let output1 = run_fft_phases(input.clone(), 100);
        let offset = 16;
        let output2 = run_fft_phases_parallel(offset, input, 100);
        assert_eq!(&output1[offset..], &output2[offset..]);
    }
}
