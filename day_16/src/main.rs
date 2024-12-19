/// Advent of Code 2024 - Day 16
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
use std::{collections::HashMap, fs};

use petgraph::{algo::dijkstra, graph::UnGraph};

#[derive(Debug, PartialEq, Eq)]
enum MapEntity {
    Wall,
    Empty,
    Start,
    End,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

fn read_puzzle_input(filename: &str) -> Vec<Vec<MapEntity>> {
    let contents = fs::read_to_string(filename).unwrap();

    let mut map = Vec::new();
    for  row in contents.trim().split('\n') {
        let mut line = Vec::new();
        for v in row.chars() {
            match v {
                '#' => line.push(MapEntity::Wall),
                '.' => line.push(MapEntity::Empty),
                'S' => line.push(MapEntity::Start),
                'E' => line.push(MapEntity::End),

                other => panic!("unknown map entity {other}"),
            }
        }
        map.push(line);
    }

    map
}

fn main() {
    let map = read_puzzle_input("input.txt");

    let mut graph: petgraph::Graph<(), i32, petgraph::Undirected> = UnGraph::new_undirected();

    let mut start_node = None;
    let mut end_node = None;

    let mut end_coordinate = None;

    let mut nodes = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            match v {
                MapEntity::Start => {
                    nodes.insert((x, y, Direction::Up), graph.add_node(()));
                    nodes.insert((x, y, Direction::Down), graph.add_node(()));
                    nodes.insert((x, y, Direction::Left), graph.add_node(()));

                    let start = graph.add_node(());
                    nodes.insert((x, y, Direction::Right), start);
                    start_node = Some(start);
                }
                MapEntity::End => {
                    let end = graph.add_node(());
                    nodes.insert((x, y, Direction::None), end);
                    end_node = Some(end);
                    end_coordinate = Some((x, y));
                }
                MapEntity::Empty => {
                    for direction in [
                        Direction::Up,
                        Direction::Down,
                        Direction::Left,
                        Direction::Right,
                    ] {
                        nodes.insert((x, y, direction), graph.add_node(()));
                    }
                }
                MapEntity::Wall => (),
            }
        }
    }

    // Add all rotations to graph
    for ((x, y, direction), node) in &nodes {
        let possible_directions = match direction {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
            Direction::None => continue, // rotating at end position is not relevant
        };

        for possible_direction in possible_directions {
            if let Some(other_node) = nodes.get(&(*x, *y, possible_direction)) {
                graph.add_edge(*node, *other_node, 1000);
            }
        }
    }

    let start_node = start_node.unwrap();
    let end_node = end_node.unwrap();

    // Add all step forward to graph
    let end_coordinate = end_coordinate.unwrap();
    for ((x, y, direction), node) in &nodes {
        let (nx, ny) = match direction {
            Direction::Up => (*x, *y - 1),
            Direction::Down => (*x, *y + 1),
            Direction::Left => (*x - 1, *y),
            Direction::Right => (*x + 1, *y),
            Direction::None => continue, // rotating at end position is not relevant
        };

        if (nx, ny) == end_coordinate {
            graph.add_edge(*node, end_node, 1);
        }
        if let Some(other_node) = nodes.get(&(nx, ny, *direction)) {
            graph.add_edge(*node, *other_node, 1);
        }
    }

    let res = dijkstra(&graph, start_node, None, |a| *a.weight());

    let result = *res.get(&end_node).unwrap();
    println!("{result}");
    assert_eq!(result, 102504);

    // Part 2

    // TODO
}
