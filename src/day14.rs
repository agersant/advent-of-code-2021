use std::{collections::HashMap, fs, mem};

use itertools::Itertools;

struct Polymer {
    pairs: HashMap<(char, char), u64>,
    counts: HashMap<char, u64>,
}

impl Polymer {
    fn new(template: &str) -> Self {
        let mut pairs = HashMap::new();
        for (a, b) in template.chars().tuple_windows() {
            *pairs.entry((a, b)).or_default() += 1;
        }
        let mut counts = HashMap::new();
        for c in template.chars() {
            *counts.entry(c).or_default() += 1;
        }
        Polymer { pairs, counts }
    }

    fn measure(&self) -> u64 {
        let (min, max) = self
            .counts
            .iter()
            .map(|(_c, n)| n)
            .minmax()
            .into_option()
            .unwrap();
        max - min
    }
}

type Rules = HashMap<(char, char), char>;

fn read_input() -> (String, Rules) {
    let raw_data = fs::read_to_string("inputs/14").unwrap();
    let mut template = String::new();
    let mut rules = Rules::new();
    for line in raw_data.lines() {
        if line.contains("->") {
            let (a, b, c) = line.replace(" -> ", "").chars().collect_tuple().unwrap();
            rules.insert((a, b), c);
        } else {
            template += line;
        }
    }
    (template, rules)
}

fn expand(mut polymer: Polymer, rules: &Rules) -> Polymer {
    let mut counts = mem::take(&mut polymer.counts);
    let mut pairs = HashMap::new();
    for ((a, b), n) in polymer.pairs {
        match rules.get(&(a, b)) {
            Some(c) => {
                *pairs.entry((a, *c)).or_default() += n;
                *pairs.entry((*c, b)).or_default() += n;
                *counts.entry(*c).or_default() += n;
            }
            None => *pairs.entry((a, b)).or_default() += n,
        }
    }
    Polymer { pairs, counts }
}

#[allow(dead_code)]
pub fn part1() {
    let (template, rules) = read_input();
    let mut polymer = Polymer::new(&template);
    for _ in 0..10 {
        polymer = expand(polymer, &rules);
    }
    println!("14.1 {}", polymer.measure());
}

#[allow(dead_code)]
pub fn part2() {
    let (template, rules) = read_input();
    let mut polymer = Polymer::new(&template);
    for _ in 0..40 {
        polymer = expand(polymer, &rules);
    }
    println!("14.2 {}", polymer.measure());
}
