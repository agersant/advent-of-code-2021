use itertools::{iproduct, Itertools};
use pathfinding::prelude::astar;
use std::fs;

fn read_input() -> Vec<Vec<u8>> {
    let raw_data = fs::read_to_string("inputs/15").unwrap();
    raw_data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect()
}

fn solve<S: FnMut((usize, usize)) -> u32>(goal: &(usize, usize), mut cost_fn: S) -> u32 {
    let (_path, cost) = astar(
        &(0, 0),
        |(x, y): &(usize, usize)| {
            iproduct!(
                x.saturating_sub(1)..=(goal.0).min(x + 1),
                y.saturating_sub(1)..=(goal.1).min(y + 1)
            )
            .filter(|&(a, b)| (a == *x) ^ (b == *y))
            .map(|(x, y)| ((x, y), cost_fn((x, y))))
            .collect_vec()
        },
        |(x, y)| (goal.0 - x + goal.1 - y) as u32,
        |p| p == goal,
    )
    .unwrap();
    cost
}

#[allow(dead_code)]
pub fn part1() {
    let maze = read_input();
    let (w, h) = (maze[0].len(), maze.len());
    let goal = (w - 1, h - 1);
    let cost_fn = |(x, y): (usize, usize)| maze[y][x] as u32;
    println!("15.1 {}", solve(&goal, cost_fn));
}

#[allow(dead_code)]
pub fn part2() {
    let maze = read_input();
    let (tile_w, tile_h) = (maze[0].len(), maze.len());
    let (w, h) = (5 * tile_w, 5 * tile_h);
    let goal = (w - 1, h - 1);
    let cost_fn = |(x, y): (usize, usize)| {
        let tile = x / tile_w + y / tile_h;
        let cost = 1 + ((maze[y % tile_h][x % tile_w] as usize + tile + 8) % 9);
        cost as u32
    };
    println!("15.2 {}", solve(&goal, cost_fn));
}
