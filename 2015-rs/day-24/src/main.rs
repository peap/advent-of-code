extern crate itertools;

use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

pub type Gift = u64;
pub type Group = Vec<Gift>;

pub fn load_gifts<'a>(filename: &'a str) -> Group {
    let mut gifts: Group = vec![];
    let f = File::open(filename).expect("Couldn't open file.");
    let reader = BufReader::new(f);
    for line in reader.lines() {
        if let Ok(text) = line {
            gifts.push(text.parse().unwrap());
        }
    }
    gifts
}

pub fn can_split_remainder(to_exclude: &Group, gifts: &Group, n: usize) -> bool {
    let mut remainder = Vec::new();
    for gift in gifts.iter() {
        if !to_exclude.contains(&gift) {
            remainder.push(gift.clone());
        }
    }
    let sum = remainder.iter().sum::<Gift>();
    if sum % n as Gift != 0 {
        return false;
    }
    let target_weight = sum / n as Gift;
    for k in 1..(remainder.len() - 1) {
        let combinations = remainder.iter().combinations(k);
        for combo in combinations {
            if combo.iter().fold(0, |acc, g| acc + **g) == target_weight {
                return true;
            }
        }
    }
    false
}

pub fn optimize_sleigh(gifts: &Group, n: usize) -> Option<(usize, Gift)> {
    let target: Gift = gifts.iter().sum::<Gift>() / n as Gift;
    let mut lowest_qe = Gift::max_value();
    for k in 1..gifts.len() {
        let combinations = gifts.iter().combinations(k);
        for combo in combinations {
            let owned_combo: Group = combo.iter().map(|g| **g).collect();
            let sum = owned_combo.iter().fold(0, |acc, g| acc + *g);
            if sum == target && can_split_remainder(&owned_combo, &gifts, n - 1) {
                lowest_qe = cmp::min(
                    lowest_qe,
                    owned_combo.iter().fold(1, |acc, g| acc * *g)
                );
            }
        }
        if lowest_qe < Gift::max_value() {
            return Some((k, lowest_qe));
        }
    }
    None
}

fn main() {
    let gifts = load_gifts("input.txt");
    // Part 1
    if let Some((min_n, min_qe)) = optimize_sleigh(&gifts, 3) {
        println!("Part 1: best 3-part arrangement has n={} and QE={}", min_n, min_qe);
    } else {
        println!("Part 1: couldn't find any sleigh arrangements");
    }
    // Part 2
    if let Some((min_n, min_qe)) = optimize_sleigh(&gifts, 4) {
        println!("Part 2: best 4-part arrangement has n={} and QE={}", min_n, min_qe);
    } else {
        println!("Part 2: couldn't find any sleigh arrangements");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splitting_remainder() {
        let to_exclude_1: Group = vec![1];
        let to_exclude_2: Group = vec![2];
        let to_exclude_3: Group = vec![4];
        let list: Group = vec![1, 2, 3, 4];
        assert!(!can_split_remainder(&to_exclude_1, &list, 2));
        assert!(can_split_remainder(&to_exclude_2, &list, 2));
        assert!(can_split_remainder(&to_exclude_3, &list, 2));
    }

    #[test]
    fn test_example_1() {
        let items = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let optimized = optimize_sleigh(&items, 3);
        assert_eq!(optimized, Some((2, 99)));
    }

    #[test]
    fn test_someone_elses_answer() {
        // https://www.reddit.com/r/adventofcode/comments/3y1s7f/day_24_solutions/cy9v5vo/
        let items = vec![
            1, 3, 5, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 67,
            71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113,
        ];
        let optimized = optimize_sleigh(&items, 3);
        assert_eq!(optimized, Some((6, 10_439_961_859)));
    }

    #[test]
    fn test_part_1() {
        let gifts = load_gifts("input.txt");
        let optimized = optimize_sleigh(&gifts, 3);
        assert_eq!(optimized, Some((6, 11846773891)));
    }

    #[test]
    fn test_example_2() {
        let items = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let optimized = optimize_sleigh(&items, 4);
        assert_eq!(optimized, Some((2, 44)));
    }

    #[test]
    fn test_part_2() {
        let gifts = load_gifts("input.txt");
        let optimized = optimize_sleigh(&gifts, 4);
        assert_eq!(optimized, Some((4, 80393059)));
    }

}
