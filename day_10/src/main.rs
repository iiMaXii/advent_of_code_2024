/// Advent of Code 2024 - Day 10
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

fn read_puzzle_input(filename: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(filename).unwrap();

    contents
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn find_edges(map: &[Vec<i32>], start_position: (usize, usize)) -> Vec<(usize, usize)> {
    let width = map.first().unwrap().len() as i32;
    let height = map.len() as i32;

    let mut result = Vec::new();

    let mut visited_nodes = HashSet::new();
    let mut to_investigate = vec![start_position];

    while let Some((x, y)) = to_investigate.pop() {
        visited_nodes.insert((x, y));

        let value = map.get(y).unwrap().get(x).unwrap();
        if *value == 9 {
            result.push((x, y));
            continue;
        }

        let x = x as i32;
        let y = y as i32;
        for (px, py) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if px < 0 || py < 0 || px >= width || py >= height {
                continue;
            }

            if visited_nodes.contains(&(px as usize, py as usize)) {
                continue;
            }

            if *map.get(py as usize).unwrap().get(px as usize).unwrap() == value + 1 {
                to_investigate.push((px as usize, py as usize));
            }
        }
    }

    result
}

fn find_edges_2(map: &[Vec<i32>], start_position: (usize, usize)) -> Vec<(usize, usize)> {
    let width = map.first().unwrap().len() as i32;
    let height = map.len() as i32;

    let mut result = Vec::new();

    let mut to_investigate = vec![start_position];

    while let Some((x, y)) = to_investigate.pop() {
        let value = map.get(y).unwrap().get(x).unwrap();
        if *value == 9 {
            result.push((x, y));
            continue;
        }

        let x = x as i32;
        let y = y as i32;
        for (px, py) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if px < 0 || py < 0 || px >= width || py >= height {
                continue;
            }

            if *map.get(py as usize).unwrap().get(px as usize).unwrap() == value + 1 {
                to_investigate.push((px as usize, py as usize));
            }
        }
    }

    result
}

fn main() {
    let map = read_puzzle_input("input.txt");

    let starting_points: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, value)| match value {
                    0 => Some((x, y)),
                    _ => None,
                })
        })
        .collect();

    let mut result = 0;
    for pos in &starting_points {
        let edges = find_edges(&map, *pos);
        result += edges.len();
    }

    println!("{result}");
    assert_eq!(result, 709);

    let mut result = 0;
    for pos in starting_points {
        let edges = find_edges_2(&map, pos);
        result += edges.len();
    }

    println!("{result}");
    assert_eq!(result, 1326);
}
