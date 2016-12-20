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

pub fn can_split_remainder(to_exclude: &Group, gifts: &Group)
        -> bool {
    let mut remainder = Vec::new();
    for gift in gifts.iter() {
        if !to_exclude.contains(&gift) {
            remainder.push(gift.clone());
        }
    }
    let sum = remainder.iter().sum::<Gift>();
    if sum % 2 != 0 {
        return false;
    }
    let target_weight = sum / 2;
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

pub fn optimize_sleigh(gifts: Group, min: usize) -> Option<(usize, Gift)> {
    let target: Gift = gifts.iter().sum::<Gift>() / 3;
    let mut lowest_qe = Gift::max_value();
    for k in min..(gifts.len() / 3) {
        let combinations = gifts.iter().combinations(k);
        for combo in combinations {
            let owned_combo: Group = combo.iter().map(|g| **g).collect();
            let sum = owned_combo.iter().fold(0, |acc, g| acc + *g);
            if sum == target && can_split_remainder(&owned_combo, &gifts) {
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
    if let Some((min_n, min_qe)) = optimize_sleigh(gifts, 6) {
        println!("Part 1: best arrangement has n={} and QE={}", min_n, min_qe);
    } else {
        println!("Part 1: couldn't find any sleigh arrangements");
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
        assert!(!can_split_remainder(&to_exclude_1, &list));
        assert!(can_split_remainder(&to_exclude_2, &list));
        assert!(can_split_remainder(&to_exclude_3, &list));
    }

    #[test]
    fn test_example_1() {
        let items = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let optimized = optimize_sleigh(items, 2);
        assert_eq!(optimized, Some((2, 99)));
    }

    #[test]
    fn test_someone_elses_answer() {
        // https://www.reddit.com/r/adventofcode/comments/3y1s7f/day_24_solutions/cy9v5vo/
        let items = vec![
            1, 3, 5, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 67,
            71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113,
        ];
        let optimized = optimize_sleigh(items, 6);
        assert_eq!(optimized, Some((6, 10_439_961_859)));
    }

    #[test]
    fn test_part_1() {
        let gifts = load_gifts("input.txt");
        let optimized = optimize_sleigh(gifts, 6);
        assert_eq!(optimized, Some((6, 11846773891)));
    }

}
