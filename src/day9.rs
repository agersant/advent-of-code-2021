use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

fn read_input() -> Cave {
    let input_file = File::open("inputs/9").unwrap();
    Cave {
        data: BufReader::new(input_file)
            .lines()
            .map(|line| {
                line.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect_vec()
            })
            .collect(),
    }
}

struct Cave {
    data: Vec<Vec<i32>>,
}

impl Cave {
    fn size(&self) -> (usize, usize) {
        (self.data[0].len(), self.data.len())
    }

    fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize, i32)> {
        let (w, h) = self.size();
        let mut result = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                let (rx, ry) = (x as i32 + dx, y as i32 + dy);
                if rx < 0 || ry < 0 {
                    continue;
                }
                let (rx, ry) = (rx as usize, ry as usize);
                if (dx == 0) == (dy == 0) || rx >= w || ry >= h {
                    continue;
                }
                result.push((rx, ry, self.data[ry][rx]));
            }
        }
        result
    }

    fn traverse(&self) -> impl Iterator<Item = (usize, usize, i32)> + '_ {
        let (w, _h) = self.size();
        self.data
            .iter()
            .flat_map(|row| row.iter())
            .enumerate()
            .map(move |(p, n)| (p % w, p / w, *n))
    }

    fn flood_fill(&mut self, height_map: &Cave, x: usize, y: usize) -> usize {
        let mut num_tiles = 0;
        let mut todo = vec![(x, y)];
        let mut done = HashSet::<(usize, usize)>::from_iter(todo.iter().copied());
        while let Some((x, y)) = todo.pop() {
            if height_map.data[y][x] == 9 {
                continue;
            }
            self.data[y][x] = 9;
            num_tiles += 1;
            for (x, y, _) in self.neighbours(x, y) {
                if !done.contains(&(x, y)) {
                    todo.push((x, y));
                    done.insert((x, y));
                }
            }
        }
        num_tiles
    }
}

#[allow(dead_code)]
pub fn part1() {
    let map = read_input();
    let result = map
        .traverse()
        .map(
            |(x, y, n)| match map.neighbours(x, y).iter().all(|&(_, _, v)| v > n) {
                true => n + 1,
                false => 0,
            },
        )
        .sum::<i32>();
    println!("day 9.1 {}", result);
}

#[allow(dead_code)]
pub fn part2() {
    let height_map = read_input();
    let (w, h) = height_map.size();
    let mut basins = Cave {
        data: vec![vec![0; w]; h],
    };
    let mut sizes = vec![];
    for (x, y, _n) in height_map.traverse() {
        if basins.data[y][x] == 0 {
            sizes.push(basins.flood_fill(&height_map, x, y));
        }
    }
    let result: usize = sizes.iter().sorted().rev().take(3).product();
    println!("day 9.2 {}", result);
}
