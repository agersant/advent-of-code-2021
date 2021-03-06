use itertools::Itertools;
use serde_scan::scan;
use std::{collections::HashMap, fs};

type Point = (usize, usize);

fn read_input() -> Vec<(Point, Point)> {
    let raw_data = fs::read_to_string("inputs/5").unwrap();
    raw_data
        .lines()
        .map(|line| -> (Point, Point) { scan!("{},{} -> {},{}" <- line).unwrap() })
        .collect()
}

fn solve(manhattan: bool) -> usize {
    let mut segments = read_input();
    if manhattan {
        segments = segments
            .into_iter()
            .filter(|(a, b)| a.0 == b.0 || a.1 == b.1)
            .collect_vec();
    }
    let mut map = HashMap::new();

    for (a, b) in &segments {
        let x_step = (b.0 as i32 - a.0 as i32).signum();
        let y_step = (b.1 as i32 - a.1 as i32).signum();
        let (mut x, mut y) = *a;
        loop {
            *map.entry((x, y)).or_insert(0) += 1;
            if (x, y) == *b {
                break;
            }
            x = (x as i32 + x_step) as usize;
            y = (y as i32 + y_step) as usize;
        }
    }

    map.values().filter(|&&v| v > 1).count()
}

#[allow(dead_code)]
pub fn part1() {
    println!("day 5.1 {}", solve(true));
}

#[allow(dead_code)]
pub fn part2() {
    println!("day 5.2 {}", solve(false));
}
