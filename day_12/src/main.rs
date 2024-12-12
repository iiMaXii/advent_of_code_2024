/// Advent of Code 2024 - Day 12
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

fn read_puzzle_input(filename: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(filename).unwrap();

    contents
        .trim()
        .split('\n')
        .map(|line| line.chars().collect())
        .collect()
}

fn find_adjacent(coordinates: &HashSet<(i32, i32)>) -> Vec<HashSet<(i32, i32)>> {
    let mut separate_gardens: Vec<HashSet<(i32, i32)>> = Vec::new();

    for coordinate in coordinates {
        let mut skip = false;
        for gard in &separate_gardens {
            if gard.contains(coordinate) {
                skip = true;
            }
        }
        if skip {
            continue;
        }

        let mut visited_coordinates = HashSet::new();
        let mut search_stack = vec![*coordinate];
        while let Some((x, y)) = search_stack.pop() {
            visited_coordinates.insert((x, y));

            let possible_next = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            for possible in possible_next {
                if !visited_coordinates.contains(&possible) && coordinates.contains(&possible) {
                    search_stack.push(possible);
                }
            }
        }
        separate_gardens.push(visited_coordinates);
    }

    separate_gardens
}

fn find_straight(
    coordinates: &HashSet<(Placement, i32, i32)>,
) -> Vec<HashSet<(Placement, i32, i32)>> {
    let mut separate_gardens: Vec<HashSet<(Placement, i32, i32)>> = Vec::new();

    for coordinate in coordinates {
        let mut skip = false;
        for gard in &separate_gardens {
            if gard.contains(coordinate) {
                skip = true;
            }
        }
        if skip {
            continue;
        }

        let mut visited_coordinates = HashSet::new();
        let mut search_stack = vec![*coordinate];
        while let Some((direction, x, y)) = search_stack.pop() {
            visited_coordinates.insert((direction, x, y));

            let possible_next = match direction {
                Placement::Top => [(direction, x - 1, y), (direction, x + 1, y)],
                Placement::Bottom => [(direction, x - 1, y), (direction, x + 1, y)],
                Placement::Left => [(direction, x, y - 1), (direction, x, y + 1)],
                Placement::Right => [(direction, x, y - 1), (direction, x, y + 1)],
            };
            for possible in possible_next {
                if !visited_coordinates.contains(&possible) && coordinates.contains(&possible) {
                    search_stack.push(possible);
                }
            }
        }
        separate_gardens.push(visited_coordinates);
    }

    separate_gardens
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Placement {
    Top,
    Bottom,
    Left,
    Right,
}
fn main() {
    let map = read_puzzle_input("input.txt");

    // Split by type (character)
    let mut areas: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match areas.get_mut(c) {
                Some(area) => {
                    area.insert((x as i32, y as i32));
                }
                None => {
                    areas.insert(*c, HashSet::from([(x as i32, y as i32)]));
                }
            }
        }
    }

    // Split adacent
    let mut plots = Vec::new();
    for coordinates in areas.values() {
        let adj = find_adjacent(coordinates);
        plots.extend_from_slice(adj.as_slice());
    }

    // Find position of fences
    let mut fences = HashMap::new();
    for (c, coordinates) in plots.iter().enumerate() {
        let mut new_fences = HashSet::new();
        for (x, y) in coordinates {
            new_fences.insert((Placement::Left, x - 1, *y));
            new_fences.insert((Placement::Right, x + 1, *y));
            new_fences.insert((Placement::Top, *x, y - 1));
            new_fences.insert((Placement::Bottom, *x, y + 1));
        }
        fences.insert(c, new_fences);
    }

    // Don't include plot land in fences
    for (c, coordinates) in plots.iter().enumerate() {
        let fence = fences.get_mut(&c).unwrap();
        for (x, y) in coordinates {
            fence.remove(&(Placement::Left, *x, *y));
            fence.remove(&(Placement::Right, *x, *y));
            fence.remove(&(Placement::Top, *x, *y));
            fence.remove(&(Placement::Bottom, *x, *y));
        }
    }

    // Calculate price
    let mut result_1 = 0;
    let mut result_2 = 0;
    for (c, area) in plots.iter().enumerate() {
        let fence = fences.get(&c).unwrap();
        result_1 += area.len() * fence.len();
        result_2 += area.len() * find_straight(fence).len();
    }

    println!("{result_1:?}");
    assert_eq!(result_1, 1450422);
    println!("{result_2:?}");
    assert_eq!(result_2, 906606);
}
