use std::{cmp::Ordering, fs};

fn read_puzzle_input(filename: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(filename).unwrap();

    contents
        .trim()
        .split('\n')
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    let first = *report.get(0).unwrap();
    let second = *report.get(1).unwrap();

    let direction: Ordering = first.cmp(&second);
    if direction == Ordering::Equal {
        return false;
    }

    for (value, next_value) in report.iter().zip(report.iter().skip(1)) {
        let current_direction = value.cmp(next_value);
        if current_direction != direction {
            return false;
        }

        let difference = (next_value - value).abs();
        if difference > 3 || difference < 1 {
            return false;
        }
    }

    true
}

/// Brute force :)
fn is_safe_report_2(report: &Vec<i32>) -> bool {
    if is_safe_report(report) {
        return true;
    }

    for (index, _) in report.iter().enumerate() {
        let mut report_copy = report.clone();
        report_copy.remove(index);
        if is_safe_report(&report_copy) {
            return true;
        }
    }

    false
}

fn main() {
    let reports = read_puzzle_input("input.txt");

    let safe_report_count = reports
        .iter()
        .map(is_safe_report)
        .filter(|is_safe| *is_safe)
        .count();
    println!("{safe_report_count}");
    assert_eq!(safe_report_count, 359);

    let safe_report_count = reports
        .iter()
        .map(is_safe_report_2)
        .filter(|is_safe| *is_safe)
        .count();
    println!("{safe_report_count}");
    assert_eq!(safe_report_count, 418);
}
