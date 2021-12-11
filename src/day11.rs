use itertools::Itertools;
use std::{collections::HashSet, fs};

fn read_input() -> Vec<Vec<u8>> {
    fs::read_to_string("inputs/11")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect_vec()
}

fn size(map: &Vec<Vec<u8>>) -> (usize, usize) {
    (map[0].len(), map.len())
}

fn neighbours_mut(
    map: &mut Vec<Vec<u8>>,
    x: usize,
    y: usize,
) -> impl Iterator<Item = (usize, usize, &mut u8)> + '_ {
    let (w, _h) = size(map);
    map.iter_mut()
        .flat_map(|row| row.iter_mut())
        .enumerate()
        .map(move |(p, n)| (p % w, p / w, n))
        .filter(move |(rx, ry, _n)| {
            (*rx as i32 - x as i32).abs() <= 1
                && (*ry as i32 - y as i32).abs() <= 1
                && (*rx != x || *ry != y)
        })
}

fn step(map: &mut Vec<Vec<u8>>) -> usize {
    let (w, h) = size(map);
    for (x, y) in (0..w).cartesian_product(0..h) {
        map[y][x] += 1;
    }
    let mut flashed = HashSet::new();
    let mut todo = (0..w).cartesian_product(0..h).collect_vec();
    while let Some((x, y)) = todo.pop() {
        if flashed.contains(&(x, y)) || map[y][x] <= 9 {
            continue;
        }
        flashed.insert((x, y));
        map[y][x] = 0;
        for (x, y, n) in neighbours_mut(map, x, y) {
            if !flashed.contains(&(x, y)) {
                *n += 1;
                todo.push((x, y));
            }
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
    let (w, h) = size(&mut map);
    let mut result = 1;
    while step(&mut map) != w * h {
        result += 1;
    }
    println!("day 11.1 {}", result);
}
