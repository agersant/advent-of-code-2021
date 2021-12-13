use itertools::Itertools;
use serde_scan::scan;
use std::{collections::HashSet, fs};

enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

fn read_input() -> (HashSet<(usize, usize)>, Vec<Fold>) {
    let raw_data = fs::read_to_string("inputs/13").unwrap();
    let mut dots = HashSet::new();
    let mut folds = Vec::new();
    for line in raw_data.lines() {
        if let Ok((x, y)) = scan!("{},{}" <- line) {
            dots.insert((x, y));
        } else if let Ok(n) = scan!("fold along x={}" <- line) {
            folds.push(Fold::Horizontal(n))
        } else if let Ok(n) = scan!("fold along y={}" <- line) {
            folds.push(Fold::Vertical(n))
        }
    }
    (dots, folds)
}

fn fold_paper(dots: HashSet<(usize, usize)>, along: &Fold) -> HashSet<(usize, usize)> {
    dots.into_iter()
        .filter_map(|(x, y)| match along {
            &Fold::Horizontal(n) if x < n => Some((x, y)),
            &Fold::Horizontal(n) if x > n => Some((2 * n - x, y)),
            &Fold::Vertical(n) if y < n => Some((x, y)),
            &Fold::Vertical(n) if y > n => Some((x, 2 * n - y)),
            _ => None,
        })
        .collect()
}

#[allow(dead_code)]
pub fn part1() {
    let (dots, folds) = read_input();
    let dots = fold_paper(dots, &folds[0]);
    println!("13.1 {}", dots.len());
}

#[allow(dead_code)]
pub fn part2() {
    let (mut dots, folds) = read_input();
    for fold in &folds {
        dots = fold_paper(dots, fold);
    }
    let width = dots.iter().map(|(x, _)| *x).max().unwrap();
    let height = dots.iter().map(|(_, y)| *y).max().unwrap();
    println!("13.2");
    for y in 0..=height {
        println!(
            "{}",
            (0..=width)
                .map(|x| if dots.contains(&(x, y)) { '█' } else { ' ' })
                .join("")
        );
    }
}
