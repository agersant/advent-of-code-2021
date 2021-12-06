#[allow(dead_code)]
static INPUT_EXAMPLE: [u8; 5] = [3, 4, 3, 1, 2];

static INPUT: [u8; 300] = [
    1, 2, 4, 5, 5, 5, 2, 1, 3, 1, 4, 3, 2, 1, 5, 5, 1, 2, 3, 4, 4, 1, 2, 3, 2, 1, 4, 4, 1, 5, 5, 1,
    3, 4, 4, 4, 1, 2, 2, 5, 1, 5, 5, 3, 2, 3, 1, 1, 3, 5, 1, 1, 2, 4, 2, 3, 1, 1, 2, 1, 3, 1, 2, 1,
    1, 2, 1, 2, 2, 1, 1, 1, 1, 5, 4, 5, 2, 1, 3, 2, 4, 1, 1, 3, 4, 1, 4, 1, 5, 1, 4, 1, 5, 3, 2, 3,
    2, 2, 4, 4, 3, 3, 4, 3, 4, 4, 3, 4, 5, 1, 2, 5, 2, 1, 5, 5, 1, 3, 4, 2, 2, 4, 2, 2, 1, 3, 2, 5,
    5, 1, 3, 3, 4, 3, 5, 3, 5, 5, 4, 5, 1, 1, 4, 1, 4, 5, 1, 1, 1, 4, 1, 1, 4, 2, 1, 4, 1, 3, 4, 4,
    3, 1, 2, 2, 4, 3, 3, 2, 2, 2, 3, 5, 5, 2, 3, 1, 5, 1, 1, 1, 1, 3, 1, 4, 1, 4, 1, 2, 5, 3, 2, 4,
    4, 1, 3, 1, 1, 1, 3, 4, 4, 1, 1, 2, 1, 4, 3, 4, 2, 2, 3, 2, 4, 3, 1, 5, 1, 3, 1, 4, 5, 5, 3, 5,
    1, 3, 5, 5, 4, 2, 3, 2, 4, 1, 3, 2, 2, 2, 1, 3, 4, 2, 5, 2, 5, 3, 5, 5, 1, 1, 1, 2, 2, 3, 1, 4,
    4, 4, 5, 4, 5, 5, 1, 4, 5, 5, 4, 1, 1, 5, 3, 3, 1, 4, 1, 3, 1, 1, 4, 1, 5, 2, 3, 2, 3, 1, 2, 2,
    2, 1, 1, 5, 1, 4, 5, 2, 4, 2, 2, 3,
];

fn solve(num_days: u32) -> u64 {
    let mut population: [u64; 9] = [0; 9];
    for i in INPUT {
        population[i as usize] += 1;
    }
    for _day in 0..num_days {
        population.rotate_left(1);
        population[6] += population[8];
    }
    population.iter().sum()
}

#[allow(dead_code)]
pub fn part1() {
    println!("day 6.1 {}", solve(80));
}

#[allow(dead_code)]
pub fn part2() {
    println!("day 6.2 {}", solve(256));
}
