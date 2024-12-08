/// Advent of Code 2024 - Day 8
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

#[derive(Debug)]
struct Antenna {
    frequency: char,
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Map {
    width: i32,
    height: i32,
    antennas: Vec<Antenna>,
}

fn read_puzzle_input(filename: &str) -> Map {
    let contents = fs::read_to_string(filename).unwrap();

    let mut antennas = Vec::new();

    for (y, line) in contents.split('\n').enumerate() {
        for (x, frequency) in line.chars().enumerate() {
            match frequency {
                '.' => (),
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    antennas.push(Antenna {
                        frequency,
                        x: x as i32,
                        y: y as i32,
                    });
                }
                other => panic!("Unexpected char {}", other),
            }
        }
    }

    let height: i32 = contents.trim().split('\n').count() as i32;
    let width: i32 = contents.trim().split('\n').next().unwrap().chars().count() as i32;

    Map {
        width,
        height,
        antennas,
    }
}

// Antinodes for part 1
fn get_antinodes(map: &Map, first: &Antenna, second: &Antenna) -> Vec<(i32, i32)> {
    let delta_x = second.x - first.x;
    let delta_y = second.y - first.y;

    // Possible antinodes from the perspective of the first antenna
    let possible_first = HashSet::from([
        (
            (first.x - delta_x, first.y - delta_y),
            (first.x + 2 * delta_x, first.y + 2 * delta_y),
        ),
        (
            (first.x + delta_x, first.y + delta_y),
            (first.x - 2 * delta_x, first.y - 2 * delta_y),
        ),
    ]);

    // Possible antinodes from the perspective of the second antenna
    let possible_second = HashSet::from([
        (
            (second.x + 2 * delta_x, second.y + 2 * delta_y),
            (second.x - delta_x, second.y - delta_y),
        ),
        (
            (second.x - 2 * delta_x, second.y - 2 * delta_y),
            (second.x + delta_x, second.y + delta_y),
        ),
    ]);

    // The intersection of these sets gives us the antinodes that fulfills the criteras for both antennas
    assert_eq!(possible_first.intersection(&possible_second).count(), 1);

    let ((x1, y1), (x2, y2)) = possible_first
        .intersection(&possible_second)
        .last()
        .unwrap();

    [(*x1, *y1), (*x2, *y2)]
        .iter()
        .filter(|(x, y)| 0 <= *x && *x < map.width && 0 <= *y && *y < map.height)
        .map(|(x, y)| (*x, *y))
        .collect()
}

// Antinodes for part 2
fn get_antinodes_2(map: &Map, first: &Antenna, second: &Antenna) -> Vec<(i32, i32)> {
    let delta_x = second.x - first.x;
    let delta_y = second.y - first.y;

    let mut antinodes = Vec::new();

    // Go backwards
    let mut x = first.x;
    let mut y = first.y;
    while 0 <= x && x < map.width && 0 <= y && y < map.height {
        antinodes.push((x, y));
        x -= delta_x;
        y -= delta_y;
    }

    // Go forwards
    let mut x = first.x + delta_x;
    let mut y = first.y + delta_y;
    while 0 <= x && x < map.width && 0 <= y && y < map.height {
        antinodes.push((x, y));
        x += delta_x;
        y += delta_y;
    }

    antinodes
}

fn main() {
    let map = read_puzzle_input("input.txt");

    // Part 1
    let mut unique_locations = HashSet::new();

    for (index, first_antenna) in map.antennas.iter().enumerate() {
        for second_antenna in map.antennas.iter().skip(index + 1) {
            if first_antenna.frequency == second_antenna.frequency {
                let antinodes = get_antinodes(&map, first_antenna, second_antenna);
                for antinode in antinodes {
                    unique_locations.insert(antinode);
                }
            }
        }
    }

    let result = unique_locations.len();
    assert_eq!(result, 361);
    println!("{result}");

    // Part 2
    let mut unique_locations = HashSet::new();

    for (index, first_antenna) in map.antennas.iter().enumerate() {
        for second_antenna in map.antennas.iter().skip(index + 1) {
            if first_antenna.frequency == second_antenna.frequency {
                let antinodes = get_antinodes_2(&map, first_antenna, second_antenna);
                for antinode in antinodes {
                    unique_locations.insert(antinode);
                }
            }
        }
    }

    let result = unique_locations.len();
    assert_eq!(result, 1249);
    println!("{result}");
}
