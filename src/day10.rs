use std::{collections::HashMap, fs};

use itertools::Itertools;

fn read_input() -> Vec<String> {
    let raw_data = fs::read_to_string("inputs/10").unwrap();
    raw_data.lines().map(|s| s.to_owned()).collect()
}

fn parse(line: &str) -> Result<Vec<char>, char> {
    let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    line.chars()
        .into_iter()
        .try_fold(Vec::new(), |mut stack, char| {
            if matches!(char, '(' | '[' | '{' | '<') {
                stack.push(char);
            } else if stack.last().map(|c| pairs[c]) == Some(char) {
                stack.pop();
            } else {
                return Err(char);
            }
            Ok(stack)
        })
}

#[allow(dead_code)]
pub fn part1() {
    let lines = read_input();
    let values = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let scores = lines
        .iter()
        .filter_map(|line| parse(line).err().map(|c| values[&c]));
    println!("day 10.1 {}", scores.sum::<u64>());
}

#[allow(dead_code)]
pub fn part2() {
    let lines = read_input();
    let values = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let scores = lines
        .iter()
        .filter_map(|line| {
            parse(line)
                .map(|stack| stack.iter().rev().fold(0_u64, |s, c| s * 5 + values[c]))
                .ok()
        })
        .sorted()
        .collect_vec();
    println!("day 10.2 {}", scores[scores.len() / 2]);
}
