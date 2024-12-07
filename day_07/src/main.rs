/// Advent of Code 2024 - Day 7
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

#[derive(Debug)]
struct Statement {
    result: i64,
    values: Vec<i64>,
}

fn read_puzzle_input(filename: &str) -> Vec<Statement> {
    let contents = fs::read_to_string(filename).unwrap();

    let mut result = Vec::new();
    for line in contents.trim().split('\n') {
        let (a, b) = line.split_once(": ").unwrap();
        result.push(Statement {
            result: a.parse().unwrap(),
            values: b.split(' ').map(|v| v.parse().unwrap()).collect(),
        });
    }

    result
}

fn calc(numbers: &Vec<i64>, operators: &[char]) -> i64 {
    assert_eq!(numbers.len(), operators.len() + 1);

    // Just insert a + in the begining, this can probably be avoided to be more efficient
    let mut operators = operators.to_owned();
    operators.insert(0, '+');
    let mut result = 0;
    for (op, value) in operators.iter().zip(numbers) {
        match *op {
            '+' => result += value,
            '*' => result *= value,

            // this operation can probably be done in a more efficient way
            '|' => {
                result = (result.to_string() + value.to_string().as_str())
                    .parse()
                    .unwrap()
            }
            _ => unreachable!(),
        }
    }

    result
}

enum Part {
    One,
    Two,
}

fn get_mutations(len: usize, part: Part) -> Vec<Vec<char>> {
    let start: Vec<char> = vec!['+'; len];

    let mut result = Vec::new();
    let mut current = start.clone();
    loop {
        result.push(current.clone());

        let mut carry_over = true;
        let mut new = Vec::new();
        for c in current.iter() {
            if carry_over {
                let (next_char, next_spill_over) = match part {
                    Part::One => match *c {
                        '+' => ('*', false),
                        '*' => ('+', true),
                        _ => unreachable!(),
                    },
                    Part::Two => match *c {
                        '+' => ('*', false),
                        '*' => ('|', false),
                        '|' => ('+', true),
                        _ => unreachable!(),
                    },
                };
                carry_over = next_spill_over;
                new.push(next_char);
            } else {
                new.push(*c);
            }
        }

        current = new;

        if current == start {
            break;
        }
    }

    result
}

fn main() {
    let expressions = read_puzzle_input("input.txt");

    let mut result = 0;
    for v in &expressions {
        for m in get_mutations(v.values.len() - 1, Part::One) {
            if v.result == calc(&v.values, &m) {
                result += v.result;
                break;
            }
        }
    }

    println!("{result}");
    assert_eq!(result, 4364915411363);

    let mut result = 0;
    for v in expressions {
        for m in get_mutations(v.values.len() - 1, Part::Two) {
            if v.result == calc(&v.values, &m) {
                result += v.result;
                break;
            }
        }
    }

    println!("{result}");
    assert_eq!(result, 38322057216320);
}
