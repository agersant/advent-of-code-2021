use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::{map_res, recognize, value},
    sequence::tuple,
    IResult,
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
    let direction = alt((
        value(Direction::Forward, tag("forward")),
        value(Direction::Down, tag("down")),
        value(Direction::Up, tag("up")),
    ));
    let distance = map_res(recognize(digit1), str::parse::<i32>);

    let parse_result: IResult<_, _> = tuple((direction, space1, distance))(input);
    let (_, (direction, _, distance)) = parse_result.unwrap();

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
