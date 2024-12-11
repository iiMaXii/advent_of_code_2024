/// Advent of Code 2024 - Day 11
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
use std::collections::HashMap;

fn count_digits(number: u64) -> u64 {
    // Always assume 1 digit, this takes care of the case where `num==0`.
    // However this special case is not relevant for this assignment since the zero case is handled before this function is called
    let mut count = 1;
    let mut number = number / 10;
    while number != 0 {
        count += 1;
        number /= 10;
    }
    count
}

// Naive solution, used in part 1. Will not work for part 2 since the stone vector will grow exponentially
fn perform_iteration(stones: &[u64]) -> Vec<u64> {
    let mut result = Vec::with_capacity(stones.len());

    for stone in stones {
        if *stone == 0 {
            result.push(1);
            continue;
        }
        let digit_count = count_digits(*stone);
        if digit_count % 2 == 0 {
            let p = 10_u64.pow(digit_count as u32 / 2);
            let left = stone / p;
            let right = stone - left * p;

            result.push(left);
            result.push(right);
            continue;
        }

        result.push(stone * 2024);
    }

    result
}

fn blink(stones: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut new_stones = HashMap::new();

    for (stone_value, stone_count) in stones {
        if *stone_value == 0 {
            new_stones.insert(1, stone_count + new_stones.get(&1).unwrap_or(&0));
            continue;
        }
        let digit_count = count_digits(*stone_value);
        if digit_count % 2 == 0 {
            let p = 10_u64.pow(digit_count as u32 / 2);
            let left = stone_value / p;
            let right = stone_value - left * p;

            new_stones.insert(left, stone_count + new_stones.get(&left).unwrap_or(&0));
            new_stones.insert(right, stone_count + new_stones.get(&right).unwrap_or(&0));

            continue;
        }

        let new_value = stone_value * 2024;
        new_stones.insert(
            new_value,
            stone_count + new_stones.get(&new_value).unwrap_or(&0),
        );
    }

    new_stones
}

fn main() {
    let stones: Vec<u64> = vec![5178527, 8525, 22, 376299, 3, 69312, 0, 275];

    // This assumes no duplicates
    let mut stone_count = HashMap::new();
    for stone in stones {
        stone_count.insert(stone, 1);
    }

    // Part 1

    for _ in 0..25 {
        //stones = perform_iteration(&stones);
        stone_count = blink(&stone_count);
    }

    let result: usize = stone_count.values().sum();
    println!("{result}");
    assert_eq!(result, 189547);

    // Part 2

    for _ in 0..50 {
        //stones = perform_iteration(&stones);
        stone_count = blink(&stone_count);
    }

    let result: usize = stone_count.values().sum();
    println!("{result}");
    assert_eq!(result, 224577979481346);
}
