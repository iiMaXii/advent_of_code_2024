/// Advent of Code 2024 - Day 15
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
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, PartialEq, Eq)]
enum MapEntity {
    Empty,
    Wall,
    Box,
    Robot,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum MapEntity2 {
    Empty,
    Wall,
    BoxStart,
    BoxEnd,
    Robot,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn read_puzzle_input(filename: &str) -> (HashMap<(usize, usize), MapEntity>, Vec<Direction>) {
    let contents = fs::read_to_string(filename).unwrap();

    let (map_str, movements) = contents.split_once("\n\n").unwrap();

    let mut map = HashMap::new();
    for (y, row) in map_str.trim().split('\n').enumerate() {
        for (x, v) in row.char_indices() {
            let entity = match v {
                '.' => MapEntity::Empty,
                '#' => MapEntity::Wall,
                'O' => MapEntity::Box,
                '@' => MapEntity::Robot,
                other => panic!("unknown map entity '{other}'"),
            };
            map.insert((x, y), entity);
        }
    }

    let movements: Vec<Direction> = movements
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            '\n' => None,
            other => panic!("unknown direction '{other}'"),
        })
        .collect();

    (map, movements)
}

fn get_next_coordinate(position: (usize, usize), direction: &Direction) -> (usize, usize) {
    let (x, y) = position;
    match direction {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

fn print_map(map: &HashMap<(usize, usize), MapEntity>) {
    let max_x = map.keys().map(|(x, _)| x).max().unwrap();
    let max_y = map.keys().map(|(_, y)| y).max().unwrap();

    for y in 0..=*max_y {
        for x in 0..=*max_x {
            match map.get(&(x, y)).unwrap() {
                MapEntity::Empty => print!("."),
                MapEntity::Wall => print!("#"),
                MapEntity::Box => print!("O"),
                MapEntity::Robot => print!("@"),
            }
        }
        println!();
    }
}

fn print_map2(map: &HashMap<(usize, usize), MapEntity2>) {
    let max_x = map.keys().map(|(x, _)| x).max().unwrap();
    let max_y = map.keys().map(|(_, y)| y).max().unwrap();

    for y in 0..=*max_y {
        for x in 0..=*max_x {
            match map.get(&(x, y)).unwrap() {
                MapEntity2::Empty => print!("."),
                MapEntity2::Wall => print!("#"),
                MapEntity2::BoxStart => print!("["),
                MapEntity2::BoxEnd => print!("]"),
                MapEntity2::Robot => print!("@"),
            }
        }
        println!();
    }
}

fn transform_map(map: &HashMap<(usize, usize), MapEntity>) -> HashMap<(usize, usize), MapEntity2> {
    let max_x = map.keys().map(|(x, _)| x).max().unwrap();
    let max_y = map.keys().map(|(_, y)| y).max().unwrap();

    let mut new_map = HashMap::new();
    for y in 0..=*max_y {
        for x in 0..=*max_x {
            match map.get(&(x, y)).unwrap() {
                MapEntity::Empty => {
                    new_map.insert((2 * x, y), MapEntity2::Empty);
                    new_map.insert((2 * x + 1, y), MapEntity2::Empty);
                }
                MapEntity::Wall => {
                    new_map.insert((2 * x, y), MapEntity2::Wall);
                    new_map.insert((2 * x + 1, y), MapEntity2::Wall);
                }
                MapEntity::Box => {
                    new_map.insert((2 * x, y), MapEntity2::BoxStart);
                    new_map.insert((2 * x + 1, y), MapEntity2::BoxEnd);
                }
                MapEntity::Robot => {
                    new_map.insert((2 * x, y), MapEntity2::Robot);
                    new_map.insert((2 * x + 1, y), MapEntity2::Empty);
                }
            }
        }
    }

    new_map
}

fn main() {
    // Part 1

    let (mut map, movements) = read_puzzle_input("input.txt");

    for direction in movements {
        // TOOD this can probably be optimized
        let robot_position = {
            let (robot_position, _) = map
                .iter()
                .find(|(_, entity)| **entity == MapEntity::Robot)
                .unwrap();

            *robot_position
        };

        let mut next = get_next_coordinate(robot_position, &direction);

        let mut stuff_to_move = vec![robot_position];
        loop {
            let next_entity = map.get(&next).unwrap();
            match next_entity {
                MapEntity::Empty => {
                    break;
                }
                MapEntity::Wall => {
                    stuff_to_move.clear();
                    break;
                }
                MapEntity::Box => {
                    stuff_to_move.push(next);
                    next = get_next_coordinate(next, &direction);
                }
                MapEntity::Robot => panic!(),
            }
        }

        stuff_to_move.reverse();
        for coordinate in stuff_to_move {
            let removed_value = map.remove(&coordinate).unwrap();
            map.insert(coordinate, MapEntity::Empty);

            let next_coordinate = get_next_coordinate(coordinate, &direction);
            map.insert(next_coordinate, removed_value);
        }
    }

    let result: usize = map
        .iter()
        .filter_map(|((x, y), entity)| match entity {
            MapEntity::Empty => None,
            MapEntity::Wall => None,
            MapEntity::Box => Some(100 * y + x),
            MapEntity::Robot => None,
        })
        .sum();

    println!("{result}");
    assert_eq!(result, 1456590);

    // Part 2

    let (map, movements) = read_puzzle_input("input.txt");
    let mut new_map = transform_map(&map);

    for direction in movements {
        // TOOD this can probably be optimized
        let robot_position = {
            let (robot_position, _) = new_map
                .iter()
                .find(|(_, entity)| **entity == MapEntity2::Robot)
                .unwrap();

            *robot_position
        };

        let next = get_next_coordinate(robot_position, &direction);

        let mut do_not_touch_again = HashSet::new();
        do_not_touch_again.insert(robot_position);
        do_not_touch_again.insert(next);

        let mut stuff_to_move = vec![robot_position];

        let mut stuff_to_check = vec![next];
        while let Some(next) = stuff_to_check.pop() {
            do_not_touch_again.insert(next);

            let next_entity = new_map.get(&next).unwrap();
            match next_entity {
                MapEntity2::Empty => {
                    // no-op
                }
                MapEntity2::Wall => {
                    stuff_to_move.clear();
                    break;
                }
                MapEntity2::BoxStart => {
                    stuff_to_move.push(next);
                    stuff_to_move.push((next.0 + 1, next.1));

                    let next_1 = get_next_coordinate(next, &direction);
                    let next_2 = get_next_coordinate((next.0 + 1, next.1), &direction);

                    if !do_not_touch_again.contains(&next_1) {
                        stuff_to_check.push(next_1);
                        do_not_touch_again.insert(next_1);
                    }
                    if !do_not_touch_again.contains(&next_2) {
                        stuff_to_check.push(next_2);
                        do_not_touch_again.insert(next_2);
                    }
                }
                MapEntity2::BoxEnd => {
                    stuff_to_move.push(next);
                    stuff_to_move.push((next.0 - 1, next.1));

                    let next_1 = get_next_coordinate(next, &direction);
                    let next_2 = get_next_coordinate((next.0 - 1, next.1), &direction);

                    if !do_not_touch_again.contains(&next_1) {
                        stuff_to_check.push(next_1);
                        do_not_touch_again.insert(next_1);
                    }
                    if !do_not_touch_again.contains(&next_2) {
                        stuff_to_check.push(next_2);
                        do_not_touch_again.insert(next_2);
                    }
                }
                MapEntity2::Robot => panic!(),
            }
        }

        let moved_stuff: Vec<((usize, usize), MapEntity2)> = stuff_to_move
            .iter()
            .map(|coordinate| {
                let stuff = new_map.get(coordinate).unwrap();
                let coordinate = get_next_coordinate(*coordinate, &direction);
                (coordinate, stuff.to_owned())
            })
            .collect();

        // Remove all the boxes
        for coordinate in stuff_to_move {
            new_map.remove(&coordinate).unwrap();
            new_map.insert(coordinate, MapEntity2::Empty);
        }

        // Insert new boxes
        for (coordinate, entity) in moved_stuff {
            new_map.insert(coordinate, entity);
        }
    }

    let result: usize = new_map
        .iter()
        .filter_map(|((x, y), entity)| match entity {
            MapEntity2::Empty => None,
            MapEntity2::Wall => None,
            MapEntity2::BoxStart => Some(100 * y + x),
            MapEntity2::BoxEnd => None,
            MapEntity2::Robot => None,
        })
        .sum();

    println!("{result}");
    assert_eq!(result, 1489116);
}
