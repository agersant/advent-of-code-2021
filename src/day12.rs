use std::{
    collections::{HashMap, HashSet},
    fs,
};

use tuple::Map;

type Maze = HashMap<String, HashSet<String>>;

struct Path {
    route: Vec<String>,
    small_caves: HashSet<String>,
    did_double_visit: bool,
}

impl Path {
    fn visit(&self, cave: &str) -> Path {
        let is_double_visit = self.small_caves.contains(cave);
        let mut new_route = self.route.clone();
        new_route.push(cave.to_owned());
        let mut new_small_caves = self.small_caves.clone();
        if cave.to_lowercase() == cave {
            new_small_caves.insert(cave.to_owned());
        }
        Path {
            route: new_route,
            small_caves: new_small_caves,
            did_double_visit: self.did_double_visit || is_double_visit,
        }
    }
}

fn read_input(name: &str) -> Maze {
    let mut maze = Maze::new();
    let raw_data = fs::read_to_string(name).unwrap();
    for line in raw_data.lines() {
        let (from, to) = line.split_once('-').unwrap().map(|s| s.to_owned());
        maze.entry(from.clone())
            .or_insert(HashSet::new())
            .insert(to.clone());
        maze.entry(to).or_insert(HashSet::new()).insert(from);
    }
    maze
}

fn explore(maze: &Maze, path: Path, allow_double: bool) -> Vec<Path> {
    let from = path.route.last().unwrap();
    if from == "end" {
        return vec![path];
    }
    let mut paths = Vec::new();
    if maze.contains_key(from) {
        for to in &maze[from] {
            if path.small_caves.contains(to)
                && !(allow_double && !path.did_double_visit && to != "start" && to != "end")
            {
                continue;
            }
            paths.extend(explore(maze, path.visit(to), allow_double));
        }
    }
    paths
}

fn compute_paths(maze: &Maze, allow_double: bool) -> Vec<Path> {
    let starting_path = Path {
        route: vec!["start".to_owned()],
        small_caves: HashSet::from(["start".to_owned()]),
        did_double_visit: false,
    };
    explore(maze, starting_path, allow_double)
}

#[allow(dead_code)]
pub fn part1() {
    let maze = read_input("inputs/12");
    println!("12.1 {}", compute_paths(&maze, false).len());
}

#[allow(dead_code)]
pub fn part2() {
    let maze = read_input("inputs/12");
    println!("12.2 {}", compute_paths(&maze, true).len());
}

#[test]
fn test_part1_example1() {
    let maze = read_input("inputs/12-example-1");
    assert_eq!(compute_paths(&maze, false).len(), 10);
}

#[test]
fn test_part1_example2() {
    let maze = read_input("inputs/12-example-2");
    assert_eq!(compute_paths(&maze, false).len(), 19);
}

#[test]
fn test_part1_example3() {
    let maze = read_input("inputs/12-example-3");
    assert_eq!(compute_paths(&maze, false).len(), 226);
}

#[test]
fn test_part2_example1() {
    let maze = read_input("inputs/12-example-1");
    assert_eq!(compute_paths(&maze, true).len(), 36);
}

#[test]
fn test_part2_example2() {
    let maze = read_input("inputs/12-example-2");
    assert_eq!(compute_paths(&maze, true).len(), 103);
}

#[test]
fn test_part2_example3() {
    let maze = read_input("inputs/12-example-3");
    assert_eq!(compute_paths(&maze, true).len(), 3509);
}
