use std::cmp;

use itertools::Itertools;

use common::{default_puzzle, Puzzle};

pub type Gift = u64;
pub type Group = Vec<Gift>;
pub type GroupArg<'a> = &'a [Gift];

pub fn can_split_remainder(to_exclude: GroupArg, gifts: GroupArg, n: usize) -> bool {
    let mut remainder = Vec::new();
    for gift in gifts.iter() {
        if !to_exclude.contains(gift) {
            remainder.push(*gift);
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

pub fn optimize_sleigh(gifts: GroupArg, n: usize) -> Option<(usize, Gift)> {
    let target: Gift = gifts.iter().sum::<Gift>() / n as Gift;
    let mut lowest_qe = Gift::max_value();
    for k in 1..gifts.len() {
        let combinations = gifts.iter().combinations(k);
        for combo in combinations {
            // let owned_combo: Group = combo.into_iter().map(|g| *g).collect();
            let owned_combo: Group = combo.into_iter().copied().collect();
            let sum = owned_combo.iter().fold(0, |acc, g| acc + *g);
            if sum == target && can_split_remainder(&owned_combo, gifts, n - 1) {
                lowest_qe = cmp::min(lowest_qe, owned_combo.iter().fold(1, |acc, g| acc * *g));
            }
        }
        if lowest_qe < Gift::max_value() {
            return Some((k, lowest_qe));
        }
    }
    None
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("It Hangs in the Balance");
    puzzle.set_part1("quantum entanglement (3-part)", |reader| {
        let gifts = reader.parsed_lines();
        optimize_sleigh(&gifts, 3).unwrap().1
    });
    puzzle.set_part2("quantum entanglement (4-part)", |reader| {
        let gifts = reader.parsed_lines();
        optimize_sleigh(&gifts, 4).unwrap().1
    });
    puzzle
}

fn main() {
    get_puzzle().run();
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
    fn test_example_2() {
        let items = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let optimized = optimize_sleigh(&items, 4);
        assert_eq!(optimized, Some((2, 44)));
    }

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(11846773891);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(80393059);
    }

    #[test]
    fn test_someone_elses_answer() {
        // https://www.reddit.com/r/adventofcode/comments/3y1s7f/day_24_solutions/cy9v5vo/
        let items = vec![
            1, 3, 5, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 67, 71, 73, 79, 83, 89,
            97, 101, 103, 107, 109, 113,
        ];
        let optimized = optimize_sleigh(&items, 3);
        assert_eq!(optimized, Some((6, 10_439_961_859)));
    }
}
