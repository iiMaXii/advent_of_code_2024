/// Advent of Code 2024 - Day 14
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
/// GNU General Public License for more details.
///
/// You should have received a copy of the GNU General Public License
/// along with this program. If not, see <https://www.gnu.org/licenses/>.
use std::fs;

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

fn read_puzzle_input(filename: &str) -> Vec<Robot> {
    let contents = fs::read_to_string(filename).unwrap();

    let re = Regex::new(r"^p=(?<px>-?\d+),(?<py>-?\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)$").unwrap();

    let mut robots = Vec::new();
    for line in contents.trim().split('\n') {
        let Some(caps) = re.captures(line) else {
            panic!()
        };
        robots.push(Robot {
            position: (
                caps.name("px").unwrap().as_str().parse().unwrap(),
                caps.name("py").unwrap().as_str().parse().unwrap(),
            ),
            velocity: (
                caps.name("vx").unwrap().as_str().parse().unwrap(),
                caps.name("vy").unwrap().as_str().parse().unwrap(),
            ),
        });
    }

    robots
}

fn print_robots(robots: &[Robot], width: i64, height: i64) {
    for y in 0..height {
        for x in 0..width {
            let count = robots
                .iter()
                .filter(|r| r.position.0 == x && r.position.1 == y)
                .count();
            assert!(count < 10);
            match count {
                0 => print!("."),
                c => print!("{c}"),
            }
        }
        println!();
    }
}

fn is_equal(lhs: &[Robot], rhs: &[Robot]) -> bool {
    for (lhs, rhs) in lhs.iter().zip(rhs) {
        if lhs != rhs {
            return false;
        }
    }

    true
}

fn main() {
    let mut robots = read_puzzle_input("input.txt");

    let width = robots.iter().map(|r| r.position.0).max().unwrap() + 1;
    let height = robots.iter().map(|r| r.position.1).max().unwrap() + 1;

    let robots_start_positions = robots.clone();

    // Part 1

    for i in 0..100 {
        robots = robots
            .into_iter()
            .map(|r| Robot {
                position: (
                    (r.position.0 + r.velocity.0).rem_euclid(width),
                    (r.position.1 + r.velocity.1).rem_euclid(height),
                ),
                velocity: r.velocity,
            })
            .collect();

        if is_equal(&robots, &robots_start_positions) {
            panic!("at iteration {i}")
        }
    }

    let middle_x = width / 2;
    let middle_y = height / 2;

    let count_1: i64 = robots
        .iter()
        .filter_map(|r| {
            if r.position.0 < middle_x && r.position.1 < middle_y {
                Some(1)
            } else {
                None
            }
        })
        .sum();

    let count_2: i64 = robots
        .iter()
        .filter_map(|r| {
            if r.position.0 > middle_x && r.position.1 < middle_y {
                Some(1)
            } else {
                None
            }
        })
        .sum();

    let count_3: i64 = robots
        .iter()
        .filter_map(|r| {
            if r.position.0 < middle_x && r.position.1 > middle_y {
                Some(1)
            } else {
                None
            }
        })
        .sum();

    let count_4: i64 = robots
        .iter()
        .filter_map(|r| {
            if r.position.0 > middle_x && r.position.1 > middle_y {
                Some(1)
            } else {
                None
            }
        })
        .sum();

    let result = count_1 * count_2 * count_3 * count_4;
    println!("{result}");
    assert_eq!(result, 224554908);

    // Part 2

    // No fancy solution here.
    // We print out all arrangements that have at least 16 robots along the middle vertially.
    // The christmas tree can be found on iteration 6544, see `output.txt`

    let middle_x = width / 2;

    for i in 0..1000000 {
        robots = robots
            .into_iter()
            .map(|r| Robot {
                position: (
                    (r.position.0 + r.velocity.0).rem_euclid(width),
                    (r.position.1 + r.velocity.1).rem_euclid(height),
                ),
                velocity: r.velocity,
            })
            .collect();

        let count = robots.iter().filter(|r| r.position.0 == middle_x).count();

        if count > 16 {
            println!("==== ITERATION {} ====", i + 1);
            print_robots(&robots, width, height);
            println!("====================")
        }

        if is_equal(&robots, &robots_start_positions) {
            panic!("converge at {i}")
        }
    }
}
