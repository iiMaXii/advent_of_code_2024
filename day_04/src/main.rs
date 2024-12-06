/// Advent of Code 2024 - Day 4
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

fn read_puzzle_input(filename: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(filename).unwrap();

    let mut matrix = Vec::new();
    for line in contents.trim().split('\n') {
        let mut matrix_line = Vec::new();
        for c in line.chars() {
            matrix_line.push(c);
        }
        matrix.push(matrix_line);
    }

    matrix
}

fn rotate_ccw(matrix: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    let max_x = matrix.first().unwrap().len() - 1;

    for _ in matrix.first().unwrap() {
        result.push(Vec::new());
    }

    for row in matrix.iter() {
        for (x, c) in row.iter().enumerate() {
            result.get_mut(max_x - x).unwrap().push(*c);
        }
    }

    result
}

fn has_word_at_coordinates(
    matrix: &[Vec<char>],
    coordinates: &[(usize, usize)],
    word: &str,
) -> bool {
    assert_eq!(coordinates.len(), word.len());

    for ((x, y), expected_c) in coordinates.iter().zip(word.chars()) {
        let c = matrix.get(*y).and_then(|line| line.get(*x));
        if let Some(c) = c {
            if *c != expected_c {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

fn main() {
    let matrix = read_puzzle_input("input.txt");

    let mut result_1 = 0;
    let mut result_2 = 0;

    let mut matrix = matrix;
    for _ in 0..4 {
        for (y, row) in matrix.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if has_word_at_coordinates(
                    &matrix,
                    &[(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
                    "XMAS",
                ) {
                    result_1 += 1;
                }

                if has_word_at_coordinates(
                    &matrix,
                    &[(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)],
                    "XMAS",
                ) {
                    result_1 += 1;
                }

                if has_word_at_coordinates(
                    &matrix,
                    &[(x + 2, y), (x + 1, y + 1), (x, y + 2)],
                    "MAS",
                ) && has_word_at_coordinates(
                    &matrix,
                    &[(x, y), (x + 1, y + 1), (x + 2, y + 2)],
                    "MAS",
                ) {
                    result_2 += 1;
                }
            }
        }
        matrix = rotate_ccw(&matrix);
    }

    // Part 1
    println!("{result_1}");
    assert_eq!(result_1, 2496);

    // Part 2
    println!("{result_2}");
    assert_eq!(result_2, 1967);
}
