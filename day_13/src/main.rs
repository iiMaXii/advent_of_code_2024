/// Advent of Code 2024 - Day 13
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

#[derive(Debug)]
struct ClawMachine {
    button_a_position: (i64, i64),
    button_b_position: (i64, i64),
    prize_position: (i64, i64),
}

fn read_puzzle_input(filename: &str) -> Vec<ClawMachine> {
    let content = fs::read_to_string(filename).unwrap();

    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let mut result = Vec::new();
    for (_, [button_a_x, button_a_y, button_b_x, button_b_y, prize_x, prize_y]) in
        re.captures_iter(&content).map(|c| c.extract())
    {
        result.push(ClawMachine {
            button_a_position: (button_a_x.parse().unwrap(), button_a_y.parse().unwrap()),
            button_b_position: (button_b_x.parse().unwrap(), button_b_y.parse().unwrap()),
            prize_position: (
                prize_x.parse::<i64>().unwrap(),
                prize_y.parse::<i64>().unwrap(),
            ),
        });
    }

    result
}

/// There is only one input combination that works.
///
/// Example:
/// ```
///    Button A: X+45, Y+76
///    Button B: X+84, Y+14
///    Prize: X=9612, Y=4342
/// ```
///
/// We get the following equation system:
///
/// ```
///    45 * a + 84 * b = 9612
///    76 * a + 14 * b = 4342
/// ```
///
/// This equation system only has only solution. But we want to generalize this, so instead consider the generalized equation system:
///
/// ```
///    X_a * a + X_b * b = X_p
///    Y_a * a + Y_b * b = Y_p
/// ```
///
/// Solving for `a`` in both equations gives us:
///
/// ```
///    a = (X_p - X_b * b) / X_a
///    a = (Y_p - Y_b * b) / Y_a
/// ```
///
/// From this we can solve for `b`:
///
/// ```
///    (X_p - X_b * b) / X_a = (Y_p - Y_b * b) / Y_a
///                         <=>
///    Y_a * (X_p - X_b * b) = X_a * (Y_p - Y_b * b)
///                         <=>
///    Y_a * X_p - Y_a * X_b * b = X_a * Y_p - X_a * Y_b * b
///                         <=>
///    Y_a * X_p - X_a * Y_p = Y_a * X_b * b - X_a * Y_b * b
///                         <=>
///    b = (Y_a * X_p - X_a * Y_p) / (Y_a * X_b * b - X_a * Y_b)
/// ```
///
/// And now we have the solution `a` and `b`:
///
/// ```
///    b = (Y_a * X_p - X_a * Y_p) / (Y_a * X_b * b - X_a * Y_b)
///    a = (X_p - X_b * b) / X_a
/// ```
fn calculate_presses(machine: &ClawMachine) -> Option<(i64, i64)> {
    let (xa, ya) = machine.button_a_position;
    let (xb, yb) = machine.button_b_position;
    let (xp, yp) = machine.prize_position;

    // Special cases for
    assert_ne!(xa, 0);
    assert_ne!(ya, 0);
    assert_ne!(xb, 0);
    assert_ne!(yb, 0);
    assert_ne!(ya * xb - xa * yb, 0);

    let b_dividend = ya * xp - xa * yp;
    let b_divisor = ya * xb - xa * yb;

    if b_dividend % b_divisor != 0 {
        return None;
    }

    let b = b_dividend / b_divisor;

    let a_dividend = xp - b * xb;
    let a_divisor = xa;

    if a_dividend % a_divisor != 0 {
        return None;
    }

    let a = a_dividend / a_divisor;

    Some((a, b))
}

fn main() {
    let machines = read_puzzle_input("input.txt");

    // Part 1
    let mut result = 0;
    for machine in &machines {
        if let Some((a, b)) = calculate_presses(machine) {
            result += a * 3 + b;
        }
    }

    println!("{result}");
    assert_eq!(result, 29711);

    // Part 2
    let machines_2: Vec<ClawMachine> = machines
        .iter()
        .map(|c| ClawMachine {
            button_a_position: c.button_a_position,
            button_b_position: c.button_b_position,
            prize_position: (
                c.prize_position.0 + 10_000_000_000_000,
                c.prize_position.1 + 10_000_000_000_000,
            ),
        })
        .collect();

    let mut result_2 = 0;
    for machine in machines_2 {
        if let Some((a, b)) = calculate_presses(&machine) {
            result_2 += a * 3 + b;
        }
    }

    println!("{result_2}");
    assert_eq!(result_2, 94955433618919);
}
