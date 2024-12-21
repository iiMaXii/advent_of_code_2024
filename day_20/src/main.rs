use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq)]
enum MapEntity {
    Wall,
    Empty,
}

fn read_puzzle_input(filename: &str) -> (Vec<Vec<MapEntity>>, (usize, usize), (usize, usize)) {
    let contents = fs::read_to_string(filename).unwrap();

    let mut start = None;
    let mut end = None;
    let mut map = Vec::new();
    for (y, row) in contents.trim().split('\n').enumerate() {
        let mut line = Vec::new();
        for (x, v) in row.char_indices() {
            match v {
                '#' => line.push(MapEntity::Wall),
                '.' => line.push(MapEntity::Empty),
                'S' => {
                    line.push(MapEntity::Empty);
                    start = Some((x, y));
                }
                'E' => {
                    line.push(MapEntity::Empty);
                    end = Some((x, y));
                }

                other => panic!("unknown map entity {other}"),
            }
        }
        map.push(line);
    }

    (map, start.unwrap(), end.unwrap())
}

fn count_cheats(
    path: &HashMap<(usize, usize), usize>,
    cheat_distance: usize,
    min_saved_time: i64,
) -> usize {
    let mut counter = 0;
    for ((x1, y1), time1) in path {
        for ((x2, y2), time2) in path {
            let cheat_time = x1.abs_diff(*x2) + y1.abs_diff(*y2);

            if cheat_time <= cheat_distance {
                let time_save = *time2 as i64 - (cheat_time as i64 + *time1 as i64);

                if time_save >= min_saved_time {
                    counter += 1;
                }
            }
        }
    }

    counter
}

fn main() {
    let (map, start, end) = read_puzzle_input("input.txt");

    let mut path = HashMap::new();
    let mut position = end;
    let mut time = 0;

    path.insert(end, time);
    while position != start {
        time += 1;
        let (x, y) = position;
        for (nx, ny) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if *map.get(ny).unwrap().get(nx).unwrap() == MapEntity::Empty
                && !path.contains_key(&(nx, ny))
            {
                position = (nx, ny);
                break;
            }
        }
        path.insert(position, time);
    }

    // Part 1

    let result = count_cheats(&path, 2, 100);
    println!("{result}");
    assert_eq!(result, 1358);

    // Part 2

    let result = count_cheats(&path, 20, 100);
    println!("{result}");
    assert_eq!(result, 1005856);
}
