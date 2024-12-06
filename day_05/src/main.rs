/// Advent of Code 2024 - Day 5
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
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
};

fn read_puzzle_input(filename: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let contents = fs::read_to_string(filename).unwrap();

    let mut ordering: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut iter = contents.split('\n');
    let mut order = iter.next().unwrap();
    while !order.is_empty() {
        let [lhs, rhs] = order.split('|').collect::<Vec<&str>>()[..] else {
            panic!()
        };
        let lhs: i32 = lhs.parse().unwrap();
        let rhs: i32 = rhs.parse().unwrap();
        if let Some(order) = ordering.get_mut(&lhs) {
            order.push(rhs);
        } else {
            ordering.insert(lhs, vec![rhs]);
        }
        order = iter.next().unwrap();
    }

    let mut seq = iter.next().unwrap();
    let mut sequences = Vec::new();
    while !seq.is_empty() {
        sequences.push(seq.split(',').map(|v| v.parse::<i32>().unwrap()).collect());
        seq = iter.next().unwrap();
    }

    (ordering, sequences)
}

fn is_valid_sequence(sequence: &[i32], ordering: &HashMap<i32, Vec<i32>>) -> bool {
    for (index, value) in sequence.iter().enumerate() {
        if let Some(after_values) = ordering.get(value) {
            for after_value in after_values {
                if let Some(after_index) = sequence.iter().position(|v| *v == *after_value) {
                    if index > after_index {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn is_less(lhs: i32, rhs: i32, ordering: &HashMap<i32, Vec<i32>>, sequence: &[i32]) -> bool {
    let mut visited_numbers = HashSet::new();
    let mut node_stack = vec![ordering.get(&lhs).unwrap()];

    while let Some(current_node) = node_stack.pop() {
        for child_node in current_node {
            assert_ne!(*child_node, lhs);
            if *child_node == rhs {
                return true;
            }

            if sequence.contains(child_node) && !visited_numbers.contains(child_node) {
                // TODO can this unwrap?
                node_stack.push(ordering.get(child_node).unwrap());
                visited_numbers.insert(child_node);
            }
        }
    }

    false
}

fn main() {
    let (ordering, sequences) = read_puzzle_input("input.txt");

    let mut result_1 = 0;
    let mut result_2 = 0;

    for sequence in sequences {
        if is_valid_sequence(&sequence, &ordering) {
            result_1 += sequence.get(sequence.len() / 2).unwrap();
        } else {
            let mut sequence_sorted = sequence.clone();
            sequence_sorted.sort_by(|lhs, rhs| {
                if is_less(*lhs, *rhs, &ordering, &sequence) {
                    Ordering::Less
                } else if is_less(*rhs, *lhs, &ordering, &sequence) {
                    Ordering::Greater
                } else {
                    panic!()
                }
            });
            result_2 += sequence_sorted.get(sequence_sorted.len() / 2).unwrap();
        }
    }

    // Part 1
    println!("{result_1}");
    assert_eq!(result_1, 6051);

    // Part 2
    println!("{result_2}");
    assert_eq!(result_2, 5093);
}
