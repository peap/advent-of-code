extern crate itertools;

use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

pub type Gift = u32;
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
    gifts.sort_by(|a, b| b.cmp(&a));
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
    let sum = remainder.iter().sum::<u32>();
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

pub fn optimize_sleigh(gifts: Group, min: usize) -> Option<(usize, u32)> {
    let target_weight: u32 = gifts.iter().sum::<u32>() / 3;
    let mut lowest_qe = u32::max_value();
    for k in min..(gifts.len() / 3) {
        let combinations = gifts.iter().combinations(k);
        for combo in combinations {
            let combo: Group = combo.iter().map(|g| **g).collect();
            let sum = combo.iter().fold(0, |acc, g| acc + *g);
            if sum == target_weight && can_split_remainder(&combo, &gifts) {
                lowest_qe = cmp::min(lowest_qe, combo.iter().fold(1, |acc, g| {
                    acc * *g
                }));
            }
        }
        if lowest_qe < u32::max_value() {
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
        let min_qe = optimize_sleigh(items, 2);
        assert_eq!(min_qe, Some((2, 99)));
    }

   #[test]
   fn test_part_1() {
       let gifts = load_gifts("input.txt");
       if let Some((min_n, min_qe)) = optimize_sleigh(gifts, 6) {
           assert!(min_qe > 25060087);
       }
   }

}
