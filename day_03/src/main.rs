/// Advent of Code 2024 - Day 3
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
use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // Part 1
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let result: i32 = re
        .captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [lhs, rhs])| lhs.parse::<i32>().unwrap() * rhs.parse::<i32>().unwrap())
        .sum();

    println!("{result}");

    // Part 2
    let re =
        Regex::new(r"(mul\((?<lhs>\d+),(?<rhs>\d+)\)|(?<do>do)\(\)|(?<dont>don\'t)\(\))").unwrap();

    let mut enabled = true;
    let mut result = 0;
    for cap in re.captures_iter(&input) {
        if let (Some(lhs), Some(rhs)) = (cap.name("lhs"), cap.name("rhs")) {
            let lhs = lhs.as_str().parse::<i32>().unwrap();
            let rhs = rhs.as_str().parse::<i32>().unwrap();

            if enabled {
                result += lhs * rhs;
            }
        } else if let Some(_) = cap.name("do") {
            enabled = true;
        } else if let Some(_) = cap.name("dont") {
            enabled = false;
        } else {
            unreachable!();
        }
    }

    println!("{result}");
}
