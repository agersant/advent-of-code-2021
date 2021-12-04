use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

fn read_input() -> (Vec<u8>, Vec<Board>) {
    let input_file = File::open("inputs/4").unwrap();
    let mut lines = BufReader::new(input_file).lines();

    let guesses = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect_vec();

    let mut boards: Vec<Board> = vec![];
    for board_string in lines
        .map(|l| l.unwrap())
        .filter(|l| l.len() > 0)
        .chunks(5)
        .into_iter()
        .map(|d| d.collect_vec())
    {
        let mut board = Board::default();
        for i in 0..5 {
            board.data[i] = board_string[i]
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect_vec()
                .as_slice()
                .try_into()
                .unwrap();
        }
        boards.push(board)
    }

    (guesses, boards)
}

#[derive(Debug, Default)]
struct Board {
    data: [[u8; 5]; 5],
    checked: [[bool; 5]; 5],
}

impl Board {
    fn guess(&mut self, g: u8) {
        for y in 0..5 {
            for x in 0..5 {
                if self.data[y][x] == g {
                    self.checked[y][x] = true;
                }
            }
        }
    }

    fn compute_score(&self, last_guess: u8) -> u32 {
        let mut score = 0;
        for y in 0..5 {
            for x in 0..5 {
                if !self.checked[y][x] {
                    score += self.data[y][x] as u32;
                }
            }
        }
        score * last_guess as u32
    }

    fn is_winning(&self) -> bool {
        let mut row_totals = [0; 5];
        let mut column_totals = [0; 5];
        for y in 0..5 {
            for x in 0..5 {
                if self.checked[y][x] {
                    row_totals[y] += 1;
                    column_totals[x] += 1;
                }
            }
        }
        *column_totals.iter().max().unwrap() == 5 || *row_totals.iter().max().unwrap() == 5
    }

    fn try_win(&self, last_guess: u8) -> Option<u32> {
        if self.is_winning() {
            Some(self.compute_score(last_guess))
        } else {
            None
        }
    }
}

#[allow(dead_code)]
pub fn part1() {
    let (guesses, mut boards) = read_input();
    for guess in guesses {
        for board in &mut boards {
            board.guess(guess);
            if let Some(score) = board.try_win(guess) {
                println!("day 4.1 {}", score);
                return;
            }
        }
    }
}

#[allow(dead_code)]
pub fn part2() {
    let (guesses, mut boards) = read_input();
    for guess in guesses {
        for i in (0..boards.len()).rev() {
            let board = &mut boards[i];
            board.guess(guess);
            if let Some(score) = board.try_win(guess) {
                if boards.len() == 1 {
                    println!("day 4.2 {}", score);
                    return;
                } else {
                    boards.remove(i);
                }
            }
        }
    }
}
