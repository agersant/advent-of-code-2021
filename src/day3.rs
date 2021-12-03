use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

fn read_input() -> Vec<Vec<bool>> {
    let input_file = File::open("inputs/3").unwrap();
    BufReader::new(input_file)
        .lines()
        .map(|line| {
            let line_content = line.unwrap();
            line_content.chars().map(|c| c == '1').collect()
        })
        .collect()
}

#[allow(dead_code)]
pub fn part1() {
    let lines = read_input();
    let num_digits = lines[0].len();
    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;
    for i in 0..num_digits {
        let num_ones = lines.iter().filter(|l| l[i]).count();
        let num_zeroes = lines.len() - num_ones;
        let contribution = 1 << (num_digits - 1 - i);
        if num_ones > num_zeroes {
            gamma_rate += contribution;
        } else {
            epsilon_rate += contribution;
        }
    }
    println!("day 3.1 {}", gamma_rate * epsilon_rate);
}

fn binary_to_integer(input: &Vec<bool>) -> u32 {
    let mut result: u32 = 0;
    let num_digits = input.len();
    for i in 0..num_digits {
        if input[i] {
            result += 1 << (num_digits - 1 - i);
        }
    }
    result
}

fn filter_candidates(candidates: Vec<&Vec<bool>>, digit: usize, invert: bool) -> Vec<&Vec<bool>> {
    if candidates.len() <= 1 {
        return candidates;
    }
    let ones = candidates
        .to_owned()
        .into_iter()
        .filter(|l| l[digit])
        .collect_vec();
    let zeroes = candidates
        .to_owned()
        .into_iter()
        .filter(|l| !l[digit])
        .collect_vec();
    if (ones.len() >= zeroes.len()) ^ invert {
        ones
    } else {
        zeroes
    }
}

#[allow(dead_code)]
pub fn part2() {
    let lines = read_input();
    let num_digits = lines[0].len();
    let mut oxygen_candidates: Vec<&Vec<bool>> = lines.iter().collect();
    let mut co2_candidates: Vec<&Vec<bool>> = lines.iter().collect();
    for i in 0..num_digits {
        oxygen_candidates = filter_candidates(oxygen_candidates, i, false);
        co2_candidates = filter_candidates(co2_candidates, i, true);
    }
    let oxygen_reading = binary_to_integer(oxygen_candidates[0]);
    let co2_reading = binary_to_integer(co2_candidates[0]);
    println!("day 3.2 {}", oxygen_reading * co2_reading);
}
