use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

fn read_input() -> Vec<i32> {
    let input_file = File::open("inputs/1").unwrap();
    let buf_reader = BufReader::new(input_file);
    buf_reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn part1() {
    let readings = read_input();
    let result = readings
        .iter()
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count();
    println!("day 1.1 {}", result);
}

#[allow(dead_code)]
pub fn part2() {
    let readings = read_input();
    let result = readings
        .iter()
        .tuple_windows()
        .filter(|(a, _b, _c, d)| d > a)
        .count();
    println!("day 1.2 {}", result);
}
