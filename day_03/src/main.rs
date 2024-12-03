use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // Part 1
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let result: i32 = re
        .captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [lhs, rhs])| lhs.parse::<i32>().unwrap() * rhs.parse::<i32>().unwrap())
        .sum();

    println!("{result}");

    // Part 2
    let re =
        Regex::new(r"(mul\((?<lhs>\d+),(?<rhs>\d+)\)|(?<do>do)\(\)|(?<dont>don\'t)\(\))").unwrap();

    let mut enabled = true;
    let mut result = 0;
    for cap in re.captures_iter(&input) {
        if let (Some(lhs), Some(rhs)) = (cap.name("lhs"), cap.name("rhs")) {
            let lhs = lhs.as_str().parse::<i32>().unwrap();
            let rhs = rhs.as_str().parse::<i32>().unwrap();

            if enabled {
                result += lhs * rhs;
            }
        } else if let Some(_) = cap.name("do") {
            enabled = true;
        } else if let Some(_) = cap.name("dont") {
            enabled = false;
        } else {
            unreachable!();
        }
    }

    println!("{result}");
}
