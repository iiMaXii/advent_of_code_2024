/// Advent of Code 2024 - Day 6
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
use std::{collections::HashSet, fs};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct Guard {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Guard {
    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn peek_step(&self) -> (i32, i32) {
        match self.direction {
            Direction::North => (self.x, self.y - 1),
            Direction::East => (self.x + 1, self.y),
            Direction::South => (self.x, self.y + 1),
            Direction::West => (self.x - 1, self.y),
        }
    }

    fn take_step(&mut self) {
        let (x, y) = self.peek_step();
        self.x = x;
        self.y = y;
    }

    fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Clone)]
struct PlayField {
    width: i32,
    height: i32,
    obstructions: HashSet<(i32, i32)>,
    guard: Guard,
}

fn read_puzzle_input(filename: &str) -> PlayField {
    let contents = fs::read_to_string(filename).unwrap();

    let mut obstructions = HashSet::new();
    let mut guard = None;

    for (y, line) in contents.trim().split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    obstructions.insert((x as i32, y as i32));
                }
                '.' => (),
                '^' => {
                    assert!(guard.is_none());

                    guard = Some(Guard {
                        x: x as i32,
                        y: y as i32,
                        direction: Direction::North,
                    })
                }
                c => panic!("{c}"),
            }
        }
    }

    let height: i32 = contents.trim().split('\n').count() as i32;
    let width: i32 = contents.trim().split('\n').next().unwrap().chars().count() as i32;

    PlayField {
        width,
        height,
        obstructions,
        guard: guard.unwrap(),
    }
}

/// Returns a set of visited x and y positions if the guard will exit the play field, will return None if the guard is stuck in an infinite loop.
fn get_visited_positions(play_field: &PlayField) -> Option<HashSet<(i32, i32)>> {
    let mut guard = play_field.guard.clone();
    let mut visited_positions = HashSet::new();
    let mut visited_positions_with_direction = HashSet::new();

    loop {
        // Check if we are stuck
        if visited_positions_with_direction.contains(&(guard.position(), guard.direction.clone())) {
            return None;
        }

        let next_position = guard.peek_step();
        if play_field.obstructions.contains(&next_position) {
            // TODO assert on 360 no-scope?
            guard.turn();
            continue;
        }

        visited_positions.insert(guard.position());
        visited_positions_with_direction.insert((guard.position(), guard.direction.clone()));

        let (next_x, next_y) = next_position;
        if next_x < 0 || next_y < 0 || next_x >= play_field.width || next_y >= play_field.height {
            break;
        }

        guard.take_step();
    }

    Some(visited_positions)
}

fn main() {
    let mut play_field = read_puzzle_input("input.txt");

    assert!(!play_field
        .obstructions
        .contains(&play_field.guard.position()));

    // Part 1

    let mut visited_nodes = get_visited_positions(&play_field).unwrap();
    println!("{}", visited_nodes.len());
    assert_eq!(visited_nodes.len(), 5080);

    // Part 2
    // This might take some time to run in debug mode, run with `--release` for faster execution

    // Remove starting position, we are not allowed to place an obstruction here
    visited_nodes.remove(&play_field.guard.position());

    let mut infinite_counter = 0;
    for (x, y) in visited_nodes {
        play_field.obstructions.insert((x, y));
        if get_visited_positions(&play_field).is_none() {
            infinite_counter += 1;
        }

        play_field.obstructions.remove(&(x, y));
    }

    println!("{infinite_counter}");
    assert_eq!(infinite_counter, 1919);
}
