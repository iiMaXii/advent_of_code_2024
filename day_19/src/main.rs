/// Advent of Code 2024 - Day 19
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

use cached::proc_macro::cached;

fn read_puzzle_input(filename: &str) -> (Vec<String>, Vec<String>) {
    let contents = fs::read_to_string(filename).unwrap();

    let (towels, combinations) = contents.trim().split_once("\n\n").unwrap();

    (
        towels.split(", ").map(|s| s.to_string()).collect(),
        combinations.split('\n').map(|s| s.to_string()).collect(),
    )
}

#[cached]
fn is_possible(combination: String, available_towels: Vec<String>) -> bool {
    if combination.is_empty() {
        return true;
    }

    for towel in &available_towels {
        let mut start_index = 0;
        while let Some(i) = combination[start_index..].find(towel) {
            let index = start_index + i;

            let left = &combination[0..index];
            let right = &combination[(index + towel.len())..];

            if is_possible(left.to_string(), available_towels.clone())
                && is_possible(right.to_string(), available_towels.clone())
            {
                return true;
            }

            start_index += i + 1;
        }
    }

    false
}

fn main() {
    let (towels, combinations) = read_puzzle_input("input.txt");

    // Part 1

    let result = combinations
        .iter()
        .map(|combination| is_possible(combination.to_string(), towels.clone()))
        .filter(|b| *b)
        .count();

    println!("{result}");
    assert_eq!(result, 365);

    // Part 2

    // TODO
}
