use std::collections::HashSet;

use itertools::iproduct;

static X_MIN: i32 = 282;
static X_MAX: i32 = 314;
static Y_MIN: i32 = -80;
static Y_MAX: i32 = -45;

#[derive(Default)]
struct Simulation {
    position: (i32, i32),
    velocity: (i32, i32),
    peak_altitude: i32,
}

impl Simulation {
    fn tick(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.velocity.0 -= self.velocity.0.signum();
        self.velocity.1 -= 1;
        self.peak_altitude = self.peak_altitude.max(self.position.1);
    }

    fn is_within_target(&self) -> bool {
        self.position.0 >= X_MIN
            && self.position.0 <= X_MAX
            && self.position.1 >= Y_MIN
            && self.position.1 <= Y_MAX
    }

    fn is_failed(&self) -> bool {
        self.position.0 > X_MAX
            || (self.velocity.0 == 0 && self.position.0 < X_MIN)
            || (self.velocity.1 < 0 && self.position.1 < Y_MIN)
    }

    fn run(&mut self) -> bool {
        loop {
            self.tick();
            if self.is_within_target() {
                return true;
            } else if self.is_failed() {
                return false;
            }
        }
    }
}

#[allow(dead_code)]
pub fn part1() {
    let mut highest_peak = i32::MIN;
    for (x, y) in iproduct!(1..=X_MAX, -1000..=1000) {
        let mut simulation = Simulation::default();
        simulation.velocity = (x, y);
        if simulation.run() {
            if simulation.peak_altitude > highest_peak {
                highest_peak = simulation.peak_altitude;
            }
        }
    }
    println!("17.1 {}", highest_peak);
}

#[allow(dead_code)]
pub fn part2() {
    let mut successful_velocities = HashSet::new();
    for (x, y) in iproduct!(1..=X_MAX, -1000..=1000) {
        let mut simulation = Simulation::default();
        simulation.velocity = (x, y);
        if simulation.run() {
            successful_velocities.insert((x, y));
        }
    }
    println!("17.2 {}", successful_velocities.len());
}
