use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_blocklist<'a>(filename: &'a str) -> Vec<(u32, u32)> {
    let mut blocklist = Vec::new();
    let f = File::open(filename).expect("Could not open file.");
    let reader = BufReader::new(f);
    for line in reader.lines() {
        match line {
            Ok(text) => {
                let mut split = text.split('-');
                let from: u32 = split.next().unwrap().parse().unwrap();
                let to: u32 = split.next().unwrap().parse().unwrap();
                blocklist.push((from, to))
            }
            Err(e) => panic!("Error reading file: {}", e),
        }
    }
    blocklist.sort_by(|a, b| (a.0).cmp(&b.0));
    // merge blocklist ranges
    let mut merged = Vec::new();
    let mut low = 0;
    let mut high = 0;
    for &(from, to) in blocklist.iter() {
        if from > high {
            merged.push((low, high));
            low = from;
            high = to;
        } else {
            high = cmp::max(high, to);
        }
    }
    merged.push((low, high));
    merged
}

pub fn find_lowest_unblocked_ip<'a>(blocklist: &'a Vec<(u32, u32)>) -> u32 {
    let mut lowest = 0;
    for &(from, to) in blocklist.iter() {
        if lowest >= from && lowest <= to {
            lowest = to + 1;
        }
    }
    lowest
}

pub fn count_unblocked_ips<'a>(blocklist: &'a Vec<(u32, u32)>, max: u32) -> u32 {
    let mut count = 0;
    let mut low = 0;
    for &(from, to) in blocklist.iter() {
        if low == 0 && from == 0 {
            low = to;
            continue;
        }
        count += (from - low) - 1;
        low = to;
    }
    count += max - low;
    count
}

fn main() {
    let blocklist = load_blocklist("input.txt");
    let lowest = find_lowest_unblocked_ip(&blocklist);
    println!("Part 1: lowest unblocked IP is {}", lowest);
    let count = count_unblocked_ips(&blocklist, u32::max_value());
    println!("Part 2: total # of unblocked IPs is {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let blocklist = vec![(0, 2), (4, 8)];
        let lowest = find_lowest_unblocked_ip(&blocklist);
        assert_eq!(lowest, 3);
        let count = count_unblocked_ips(&blocklist, 9);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_part_1() {
        let blocklist = load_blocklist("input.txt");
        let lowest = find_lowest_unblocked_ip(&blocklist);
        assert_eq!(lowest, 32259706);
    }

    #[test]
    fn test_part_2() {
        let blocklist = load_blocklist("input.txt");
        let count = count_unblocked_ips(&blocklist, u32::max_value());
        assert_eq!(count, 113);
    }
}
