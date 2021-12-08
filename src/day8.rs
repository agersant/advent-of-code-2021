use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::{Hash, Hasher},
};

use itertools::Itertools;

const LEN_TO_VALUE: [Option<u8>; 8] = [None, None, Some(1), Some(7), Some(4), None, None, Some(8)];

#[derive(PartialEq, Eq)]
struct Digit(HashSet<char>);

impl Digit {
    fn represent(&self) -> String {
        let mut chars: Vec<char> = self.0.iter().copied().collect_vec();
        chars.sort();
        chars.iter().collect()
    }
}

impl From<&str> for Digit {
    fn from(input: &str) -> Self {
        Self(input.chars().collect())
    }
}

impl Hash for Digit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.represent().hash(state);
    }
}

fn read_input() -> Vec<(Vec<Digit>, Vec<Digit>)> {
    let raw_data = fs::read_to_string("inputs/8").unwrap();
    raw_data
        .lines()
        .map(|line| -> (Vec<Digit>, Vec<Digit>) {
            let (left, right) = line.split_once(" | ").unwrap();
            let samples = left.split_ascii_whitespace().map(|s| s.into()).collect();
            let code = right.split_ascii_whitespace().map(|s| s.into()).collect();
            (samples, code)
        })
        .collect_vec()
}

#[allow(dead_code)]
pub fn part1() {
    let input = read_input();
    let result = input
        .iter()
        .flat_map(|(_, code)| code.iter().filter(|s| LEN_TO_VALUE[s.0.len()].is_some()))
        .count();
    println!("day 8.1 {}", result);
}

#[allow(dead_code)]
pub fn part2() {
    let input = read_input();

    let result: u64 = input
        .into_iter()
        .map(|(samples, code)| -> u64 {
            let mut lookups = HashMap::new();
            let mut reverse_lookups = HashMap::new();

            // Decode 1, 7, 4 and 8
            for sample in &samples {
                if let Some(n) = LEN_TO_VALUE[sample.0.len()] {
                    lookups.insert(sample.clone(), n);
                    reverse_lookups.insert(n, sample.clone());
                }
            }

            // Decode other digits
            let one = &reverse_lookups[&1].0;
            let four = &reverse_lookups[&4].0;
            let eight = &reverse_lookups[&8].0;
            for sample in &samples {
                let digit = &sample.0;
                if digit.len() == 5 {
                    if digit.is_superset(one) {
                        lookups.insert(sample.clone(), 3);
                    } else if digit
                        .union(four)
                        .copied()
                        .collect::<HashSet<char>>()
                        .eq(eight)
                    {
                        lookups.insert(sample.clone(), 2);
                    } else {
                        lookups.insert(sample.clone(), 5);
                    }
                } else if digit.len() == 6 {
                    if !digit.is_superset(one) {
                        lookups.insert(sample.clone(), 6);
                    } else if digit.is_superset(four) {
                        lookups.insert(sample.clone(), 9);
                    } else {
                        lookups.insert(sample.clone(), 0);
                    }
                }
            }

            code.iter()
                .enumerate()
                .map(|(i, d)| lookups[d] as u64 * 10_u64.pow(3 - i as u32))
                .sum()
        })
        .sum();

    println!("day 8.2 {}", result);
}
