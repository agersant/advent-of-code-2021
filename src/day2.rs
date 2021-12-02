use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone)]
enum Direction {
    Forward,
    Down,
    Up,
}

struct Instruction {
    direction: Direction,
    distance: i32,
}

fn parse_instruction(input: &str) -> Instruction {
    let (direction_str, distance_str) = input.split_once(' ').unwrap();
    let direction = match direction_str {
        "forward" => Direction::Forward,
        "down" => Direction::Down,
        _ => Direction::Up,
    };
    let distance = str::parse::<i32>(distance_str).unwrap();
    Instruction {
        direction,
        distance,
    }
}

fn read_input() -> Vec<Instruction> {
    let input_file = File::open("inputs/2").unwrap();
    BufReader::new(input_file)
        .lines()
        .map(|line| parse_instruction(&line.unwrap()))
        .collect()
}

#[allow(dead_code)]
pub fn part1() {
    let instructions = read_input();
    let (mut position, mut depth) = (0, 0);
    for instruction in instructions {
        match instruction.direction {
            Direction::Forward => position += instruction.distance,
            Direction::Down => depth += instruction.distance,
            Direction::Up => depth -= instruction.distance,
        }
    }
    println!("day 2.1 {}", position * depth);
}

#[allow(dead_code)]
pub fn part2() {
    let instructions = read_input();
    let (mut position, mut depth, mut aim) = (0, 0, 0);
    for instruction in instructions {
        match instruction.direction {
            Direction::Forward => {
                position += instruction.distance;
                depth += aim * instruction.distance;
            }
            Direction::Down => aim += instruction.distance,
            Direction::Up => aim -= instruction.distance,
        }
    }
    println!("day 2.2 {}", position * depth);
}
