/// Advent of Code 2024 - Day 9
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
use std::{borrow::BorrowMut, fs};
 
#[derive(Debug, Clone)]
struct Segment {
    length: usize,
    identifier: Option<usize>,
}
 
impl Segment {
    fn new_file(length: usize, identifier: usize) -> Segment {
        Segment {
            length,
            identifier: Some(identifier),
        }
    }
 
    fn new_empty(length: usize) -> Segment {
        Segment {
            length,
            identifier: None,
        }
    }
 
    fn is_empty(&self) -> bool {
        self.identifier.is_none()
    }
}
 
fn read_puzzle_input(filename: &str) -> Vec<Segment> {
    let contents = fs::read_to_string(filename).unwrap();
 
    contents
        .trim()
        .char_indices()
        .zip([true, false].iter().cycle())
        .map(|((index, c), is_file)| {
            if *is_file {
                Segment::new_file(c.to_digit(10).unwrap() as usize, index / 2)
            } else {
                Segment::new_empty(c.to_digit(10).unwrap() as usize)
            }
        })
        .collect()
}
 
#[derive(Debug)]
struct ReorderingInstruction {
    empty_block_index: usize,
    file_index: usize,
    new_empty_block_length: usize,
    old_file_length: usize,
    new_segment: Segment,
}
 
enum Algorithm {
    PartOne,
    PartTwo { identifier: usize },
}
 
enum InstructionError {
    NoReallocationPossible,
    EndOfFile,
}
 
/// In hindsight the todays parts should probably have been kept apart. But now the refactoring is already done
fn get_instruction(
    segments: &[Segment],
    algorithm: &Algorithm,
) -> Result<ReorderingInstruction, InstructionError> {
    let (first_empty_index, first_empty_segment, last_file_index, last_file) = match algorithm {
        Algorithm::PartOne => {
            let (first_empty_index, first_empty_segment) = match segments
                .iter()
                .enumerate()
                .find(|(_, segment)| segment.is_empty() && segment.length > 0)
            {
                Some(a) => a,
                None => return Err(InstructionError::EndOfFile),
            };
 
            let (last_file_index, last_file) = match segments
                .iter()
                .enumerate()
                .rev()
                .find(|(_, segment)| !segment.is_empty() && segment.length > 0)
            {
                Some(a) => a,
                None => return Err(InstructionError::EndOfFile),
            };
 
            if first_empty_index > last_file_index {
                return Err(InstructionError::EndOfFile);
            }
 
            (
                first_empty_index,
                first_empty_segment,
                last_file_index,
                last_file,
            )
        }
        Algorithm::PartTwo { identifier } => {
            let (last_file_index, last_file) = match segments
                .iter()
                .enumerate()
                .rev()
                .find(|(_, segment)| segment.identifier == Some(*identifier))
            {
                Some(a) => a,
                None => panic!(),
            };
 
            let (first_empty_index, first_empty_segment) = match segments
                .iter()
                .enumerate()
                .find(|(_, segment)| segment.is_empty() && segment.length >= last_file.length)
            {
                Some(a) => a,
                None => return Err(InstructionError::EndOfFile),
            };
 
            if first_empty_index > last_file_index {
                return Err(InstructionError::NoReallocationPossible);
            }
 
            (
                first_empty_index,
                first_empty_segment,
                last_file_index,
                last_file,
            )
        }
    };
 
    let new_segment_length = std::cmp::min(first_empty_segment.length, last_file.length);
    let empty_length = first_empty_segment.length - new_segment_length;
    let old_file_length = last_file.length - new_segment_length;
 
    Ok(ReorderingInstruction {
        empty_block_index: first_empty_index,
        file_index: last_file_index,
        new_empty_block_length: empty_length,
        old_file_length,
        new_segment: Segment {
            length: new_segment_length,
            identifier: last_file.identifier,
        },
    })
}
 
fn run_compactor(segments: &mut Vec<Segment>, algorithm: Algorithm) {
    let mut algorithm = algorithm;
 
    loop {
        // Remove empty blocks
        segments.retain(|segment| segment.length > 0);
 
        // Merge empty blocks
        let mut new_segments: Vec<Segment> = Vec::new();
        for segment in segments.iter_mut() {
            match (new_segments.last(), segment) {
                (Some(last_segment), segment) if last_segment.is_empty() && segment.is_empty() => {
                    new_segments.last_mut().unwrap().length += segment.length;
                }
                (_, segment) => new_segments.push(segment.clone()),
            }
        }
        *segments = new_segments;
 
        match get_instruction(segments, &algorithm) {
            Ok(instruction) => {
                segments
                    .get_mut(instruction.empty_block_index)
                    .unwrap()
                    .length = instruction.new_empty_block_length;
 
                segments.get_mut(instruction.file_index).unwrap().length =
                    instruction.old_file_length;
                segments.insert(
                    instruction.file_index,
                    Segment::new_empty(instruction.new_segment.length),
                );
 
                segments.insert(instruction.empty_block_index, instruction.new_segment);
            }
            Err(InstructionError::EndOfFile) => break,
            Err(InstructionError::NoReallocationPossible) => (),
        }
 
        if let Algorithm::PartTwo { identifier } = algorithm.borrow_mut() {
            if *identifier == 0 {
                break;
            }
 
            *identifier -= 1;
        }
    }
}
 
fn calculate_checksum(segments: &[Segment]) -> usize {
    let mut result = 0;
    let mut index_counter = 0;
    for segment in segments {
        for _ in 0..segment.length {
            result += index_counter * segment.identifier.unwrap_or(0);
            index_counter += 1;
        }
    }
    result
}

fn main() {
    let segments = read_puzzle_input("input.txt");
 
    let mut segments_1 = segments.clone();
    let mut segments_2 = segments;
 
    // Part 1
    // This might take some time to run in debug mode, run with `--release` for faster execution
 
    run_compactor(&mut segments_1, Algorithm::PartOne);
 
    let result = calculate_checksum(&segments_1);
 
    println!("{result}");
    assert_eq!(result, 6398608069280);
 
    // Part 2
 
    let last_identifier = segments_2
        .iter()
        .rev()
        .find(|segment| !segment.is_empty() && segment.length > 0)
        .unwrap()
        .identifier
        .unwrap();
 
    run_compactor(
        &mut segments_2,
        Algorithm::PartTwo {
            identifier: last_identifier,
        },
    );
 
    let result = calculate_checksum(&segments_2);
 
    println!("{result}");
    assert_eq!(result, 6427437134372);
}