/// Advent of Code 2024 - Day 18
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
use petgraph::{algo::dijkstra, graph::UnGraph};

use std::{collections::HashMap, fs};

fn read_puzzle_input(filename: &str) -> Vec<(i32, i32)> {
    let contents = fs::read_to_string(filename).unwrap();

    contents
        .trim()
        .split('\n')
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();

            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn get_minimum_steps(coordinates: &[(i32, i32)]) -> Option<i32> {
    let max_x = 70;
    let max_y = 70;

    let start = (0, 0);
    let end = (max_x, max_y);

    let mut graph = UnGraph::new_undirected();

    let mut nodes = HashMap::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if !coordinates.contains(&(x, y)) {
                nodes.insert((x, y), graph.add_node(()));
            }
        }
    }

    for ((x, y), node) in &nodes {
        for (px, py) in [(*x + 1, *y), (*x - 1, *y), (*x, *y + 1), (*x, *y - 1)] {
            if let Some(other_node) = nodes.get(&(px, py)) {
                graph.add_edge(*node, *other_node, 1);
                graph.add_edge(*other_node, *node, 1);
            }
        }
    }

    let start_node = *nodes.get(&start).unwrap();
    let end_node = *nodes.get(&end).unwrap();

    let res = dijkstra(&graph, start_node, None, |_| 1);

    res.get(&end_node).copied()
}

fn main() {
    let coordinates = read_puzzle_input("input.txt");

    // Part 1

    let (coordinates_split, _) = coordinates.split_at(1024);
    let result = get_minimum_steps(coordinates_split).unwrap();

    println!("{result}");
    assert_eq!(result, 312);

    // Part 2

    let mut result_index = None;
    for split_index in 1025..coordinates.len() {
        let (coordinates_split, _) = coordinates.split_at(split_index);
        if get_minimum_steps(coordinates_split).is_none() {
            result_index = Some(split_index - 1);
            break;
        }
    }

    let result = *coordinates.get(result_index.unwrap()).unwrap();

    println!("{result:?}");
    assert_eq!(result, (28, 26))
}
