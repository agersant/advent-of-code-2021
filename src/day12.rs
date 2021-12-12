use std::{
    collections::{HashMap, HashSet},
    fs,
};
use tuple::Map;

type Maze = HashMap<String, HashSet<String>>;

struct Path {
    route: Vec<String>,
    visits: HashMap<String, u32>,
}

impl Path {
    fn visit(&self, cave: &str) -> Path {
        let mut new_visits = self.visits.clone();
        if cave.to_lowercase() == *cave {
            *new_visits.entry(cave.to_owned()).or_insert(0) += 1;
        }
        Path {
            route: [&self.route[..], &[cave.to_owned()][..]].concat(),
            visits: new_visits,
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
            if to != "start" {
                let is_big = to.to_lowercase() != *to;
                if is_big
                    || (allow_double && path.visits.values().copied().max().unwrap_or_default() < 2)
                    || path.visits.get(to).copied().unwrap_or_default() == 0
                {
                    paths.extend(explore(maze, path.visit(to), allow_double));
                }
            }
        }
    }
    paths
}

fn compute_paths(maze: &Maze, allow_double: bool) -> Vec<Path> {
    let starting_path = Path {
        route: vec!["start".to_owned()],
        visits: HashMap::new(),
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
fn test_example1() {
    let maze = read_input("inputs/12-example-1");
    assert_eq!(compute_paths(&maze, false).len(), 10);
    assert_eq!(compute_paths(&maze, true).len(), 36);
}

#[test]
fn test_example2() {
    let maze = read_input("inputs/12-example-2");
    assert_eq!(compute_paths(&maze, false).len(), 19);
    assert_eq!(compute_paths(&maze, true).len(), 103);
}

#[test]
fn test_example3() {
    let maze = read_input("inputs/12-example-3");
    assert_eq!(compute_paths(&maze, false).len(), 226);
    assert_eq!(compute_paths(&maze, true).len(), 3509);
}
