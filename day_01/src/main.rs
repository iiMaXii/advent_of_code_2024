/// Advent of Code 2024 - Day 1
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

fn read_puzzle_input(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let contents = fs::read_to_string(filename).unwrap();

    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();

    for line in contents.trim().split('\n') {
        let &[left, right] = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            .as_slice()
        else {
            panic!()
        };

        left_vec.push(left);
        right_vec.push(right);
    }

    (left_vec, right_vec)
}

fn main() {
    let (left, right) = read_puzzle_input("input.txt");

    let mut left: Vec<i32> = left.into_iter().collect();
    let mut right: Vec<i32> = right.into_iter().collect();

    // Part 1
    left.sort();
    right.sort();

    let result_1: i32 = left
        .iter()
        .zip(&right)
        .map(|(left_value, right_value)| (right_value - left_value).abs())
        .sum();

    println!("{result_1}");
    assert_eq!(result_1, 1660292);

    // Part 2
    let result_2: i32 = left
        .iter()
        .map(|l| {
            let count = right.iter().filter(|&r| *r == *l).count() as i32;
            l * count
        })
        .sum();

    println!("{result_2}");
    assert_eq!(result_2, 22776016);
}
