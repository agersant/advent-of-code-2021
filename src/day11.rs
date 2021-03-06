use itertools::{iproduct, Itertools};
use std::{collections::HashSet, fs};

fn read_input() -> Vec<Vec<u32>> {
    fs::read_to_string("inputs/11")
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec()
}

fn size(map: &Vec<Vec<u32>>) -> (usize, usize) {
    (map[0].len(), map.len())
}

fn neighbours(map: &mut Vec<Vec<u32>>, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    let (w, h) = size(map);
    iproduct!(
        x.saturating_sub(1)..=(w - 1).min(x + 1),
        y.saturating_sub(1)..=(h - 1).min(y + 1)
    )
}

fn step(map: &mut Vec<Vec<u32>>) -> usize {
    let (w, h) = size(map);
    let mut flashed = HashSet::new();
    let mut todo = (0..w).cartesian_product(0..h).collect_vec();
    while let Some((x, y)) = todo.pop() {
        if flashed.contains(&(x, y)) {
            continue;
        }
        map[y][x] += 1;
        if map[y][x] <= 9 {
            continue;
        }
        flashed.insert((x, y));
        map[y][x] = 0;
        for (x, y) in neighbours(map, x, y) {
            todo.push((x, y));
        }
    }
    flashed.len()
}

#[allow(dead_code)]
pub fn part1() {
    let mut map = read_input();
    let mut result = 0;
    for _ in 0..100 {
        result += step(&mut map);
    }
    println!("day 11.1 {}", result);
}

#[allow(dead_code)]
pub fn part2() {
    let mut map = read_input();
    let (w, h) = size(&map);
    let mut result = 1;
    while step(&mut map) != w * h {
        result += 1;
    }
    println!("day 11.2 {}", result);
}
