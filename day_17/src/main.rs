/// Advent of Code 2024 - Day 17
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
use itertools::Itertools;
use std::{collections::HashSet, fs};

#[derive(Debug)]
enum Operand {
    Literal(i64),
    RegisterA,
    RegisterB,
    RegisterC,
}

impl Operand {
    fn from_literal_str(s: &str) -> Operand {
        match s {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" => Operand::Literal(s.parse().unwrap()),
            other => panic!("failed to parse literal operand {other}"),
        }
    }

    fn from_combo_str(s: &str) -> Operand {
        match s {
            "0" | "1" | "2" | "3" => Operand::Literal(s.parse().unwrap()),
            "4" => Operand::RegisterA,
            "5" => Operand::RegisterB,
            "6" => Operand::RegisterC,
            other => panic!("failed to parse combo operand {other}"),
        }
    }

    fn resolve(&self, register_a: i64, register_b: i64, register_c: i64) -> i64 {
        match self {
            Operand::Literal(l) => *l,
            Operand::RegisterA => register_a,
            Operand::RegisterB => register_b,
            Operand::RegisterC => register_c,
        }
    }
}

#[derive(Debug)]
enum OpCode {
    Adv(Operand),
    Bxl(Operand),
    Bst(Operand),
    Jnz(Operand),
    Bxc,
    Out(Operand),
    Bdv(Operand),
    Cdv(Operand),
}

fn read_puzzle_input(data: &str) -> (i64, i64, i64, Vec<OpCode>) {
    let (registers_str, program_str) = data.trim().split_once("\n\n").unwrap();

    let [ra, rb, rc] = registers_str.split('\n').collect::<Vec<&str>>()[..] else {
        panic!()
    };

    // TODO this assumes that they are in the correct order
    let (_, ra) = ra.split_once(": ").unwrap();
    let (_, rb) = rb.split_once(": ").unwrap();
    let (_, rc) = rc.split_once(": ").unwrap();

    let ra: i64 = ra.parse().unwrap();
    let rb: i64 = rb.parse().unwrap();
    let rc: i64 = rc.parse().unwrap();

    println!("{ra}");
    println!("{program_str}");
    let (_, program_str) = program_str.split_once(": ").unwrap();

    let mut program = Vec::new();
    for (opcode, operand) in program_str.split(',').tuples() {
        let opcode = match opcode {
            "0" => OpCode::Adv(Operand::from_combo_str(operand)),
            "1" => OpCode::Bxl(Operand::from_literal_str(operand)),
            "2" => OpCode::Bst(Operand::from_combo_str(operand)),
            "3" => OpCode::Jnz(Operand::from_literal_str(operand)),
            "4" => OpCode::Bxc,
            "5" => OpCode::Out(Operand::from_combo_str(operand)),
            "6" => OpCode::Bdv(Operand::from_combo_str(operand)),
            "7" => OpCode::Cdv(Operand::from_combo_str(operand)),
            other => panic!("unknown opcode {other}"),
        };
        program.push(opcode);
    }

    (ra, rb, rc, program)
}

fn execute(
    register_a: &mut i64,
    register_b: &mut i64,
    register_c: &mut i64,
    program: &[OpCode],
) -> Vec<i64> {
    let mut output = Vec::new();

    let mut instruction_pointer = 0;
    while let Some(opcode) = program.get(instruction_pointer) {
        match opcode {
            OpCode::Adv(operand) => {
                *register_a /=
                    2_i64.pow(operand.resolve(*register_a, *register_b, *register_c) as u32);
            }
            OpCode::Bxl(operand) => {
                *register_b ^= operand.resolve(*register_a, *register_b, *register_c);
            }
            OpCode::Bst(operand) => {
                *register_b = operand.resolve(*register_a, *register_b, *register_c) % 8;
            }
            OpCode::Jnz(operand) => {
                if *register_a != 0 {
                    instruction_pointer =
                        operand.resolve(*register_a, *register_b, *register_c) as usize;
                    continue;
                }
            }
            OpCode::Bxc => {
                *register_b ^= *register_c;
            }
            OpCode::Out(operand) => {
                let out = operand.resolve(*register_a, *register_b, *register_c) % 8;
                output.push(out);
            }
            OpCode::Bdv(operand) => {
                *register_b = *register_a
                    / 2_i64.pow(operand.resolve(*register_a, *register_b, *register_c) as u32);
            }
            OpCode::Cdv(operand) => {
                *register_c = *register_a
                    / 2_i64.pow(operand.resolve(*register_a, *register_b, *register_c) as u32);
            }
        }

        instruction_pointer += 1;
    }

    output
}

fn main() {
    let (ra, rb, rc, program) =
        read_puzzle_input(fs::read_to_string("input.txt").unwrap().as_str());

    // Part 1

    let mut register_a = ra;
    let mut register_b = rb;
    let mut register_c = rc;
    let output = execute(&mut register_a, &mut register_b, &mut register_c, &program);
    assert_eq!(output, vec![7, 1, 3, 7, 5, 1, 0, 3, 4]);

    // Part 2

    // A lot of trial and error went into this one. Basically brute force in a couple of steps:
    // 1. Brute force with a large step to find where output produces a vector of size 16
    // 2. Find first value that produces a vector of size 16
    // 3. Continue to brute force and observe that the difference between the attempts that match
    //    the first 6 numbers of the sequence matching seems to be re-occuring. The number 65536
    //    was the smallest so continue stepping by this.
    // 4. Eventually this algorithm terminated with the correct answer
    //
    // Running with --release flag speeds up the program at bit.
    //
    // There is definitely a faster and better way to find the answer :)
    //

    let expected_output = vec![2, 4, 1, 2, 7, 5, 0, 3, 4, 7, 1, 7, 5, 5, 3, 0];
    println!("expected_output={}", expected_output.len());

    // len 16 starts at 35184372088832
    let mut printed_found_correct_length = false;
    let mut previous_a = 0;
    let mut hash_set = HashSet::new();
    //for a in 35184372088832_i64..i64::MAX {
    for a in (35184351460367..i64::MAX).step_by(65536) {
        let mut register_a = a;
        let mut register_b = rb;
        let mut register_c = rc;
        let output = execute(&mut register_a, &mut register_b, &mut register_c, &program);
        if output.len() < 16 {
            continue;
        }
        if !printed_found_correct_length {
            println!("correct length found at {a}");
            printed_found_correct_length = true;
        }

        if output.len() > 16 {
            panic!(":(");
        }

        if output[..6] == expected_output[..6] {
            hash_set.insert(a - previous_a);
            // println!("[{hash_set:?}] A={a} len={} diff={} ({output:?})", output.len(), a-previous_a);
            previous_a = a;
        }

        if output == expected_output {
            println!("{a}");
            assert_eq!(a, 190384113204239);
            break;
        }
    }
}
