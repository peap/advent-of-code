use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Gift = u32;
pub type Group = Vec<Gift>;

pub fn load_gifts<'a>(filename: &'a str) -> Vec<Gift> {
    let mut gifts = vec![];
    let f = File::open(filename).expect("Couldn't open file.");
    let reader = BufReader::new(f);
    for line in reader.lines() {
        if let Ok(text) = line {
            gifts.push(text.parse().unwrap());
        }
    }
    gifts
}

pub fn get_combinations<T: Clone>(items: &Vec<T>, n: usize) -> Vec<Vec<T>> {
    let mut combinations = Vec::new();
    for (i, item) in items.iter().enumerate() {
        let remainder: Vec<T> = items[(i + 1)..]
            .into_iter().map(|t| t.clone()).collect();
        if n <= 1 {
            combinations.push(vec![item.clone()]);
        } else {
            let sub_combos = get_combinations(&remainder, n - 1);
            for vec in sub_combos.into_iter() {
                let mut combo: Vec<T> = vec![item.clone()];
                for item2 in vec.into_iter() {
                    combo.push(item2);
                }
                combinations.push(combo);
            }
        }
    }
    combinations
}

pub fn can_equally_divide_remainder(to_exclude: &Vec<Gift>, gifts: &Vec<Gift>)
        -> bool {
    let mut remainder = Vec::new();
    for gift in gifts.iter() {
        if !to_exclude.contains(gift) {
            remainder.push(gift.clone());
        }
    }
    let sum = remainder.iter().sum::<u32>();
    if sum % 2 != 0 {
        return false;
    }
    let target_weight = sum / 2;
    for k in 1..(remainder.len() - 1) {
        let combinations = get_combinations(&remainder, k);
        for combo in combinations.iter() {
            if combo.iter().sum::<u32>() == target_weight {
                return true;
            }
        }
    }
    false
}

pub fn optimize_sleigh(gifts: Vec<Gift>, min: usize) -> Option<(usize, u32)> {
    let target_weight: u32 = gifts.iter().sum::<u32>() / 3;
    let mut lowest_qe = u32::max_value();
    for k in min..(gifts.len() / 3) {
        let combinations = get_combinations(&gifts, k);
        for combo in combinations.iter() {
            let sum = combo.iter().sum::<u32>();
            if sum != target_weight {
                continue;
            }
            if !can_equally_divide_remainder(&combo, &gifts) {
                continue;
            }
            let qe = combo.iter().product::<u32>();
            if qe < lowest_qe {
                lowest_qe = qe;
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
    if let Some((min_n, min_qe)) = optimize_sleigh(gifts, 3) {
        println!("Part 1: best arrangement has n={} and QE={}", min_n, min_qe);
    } else {
        println!("Part 1: couldn't find any sleigh arrangements");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations_of_one() {
        let items = vec![1, 2, 3, 4];
        let combos1 = get_combinations(&items, 1);
        assert_eq!(combos1, vec![
            vec![1], vec![2], vec![3], vec![4]
        ]);
    }

    #[test]
    fn test_combinations_of_two() {
        let items = vec![1, 2, 3, 4];
        let combos2 = get_combinations(&items, 2);
        assert_eq!(combos2, vec![
            vec![1, 2], vec![1, 3], vec![1, 4],
            vec![2, 3], vec![2, 4],
            vec![3, 4],
        ]);
    }

    #[test]
    fn test_combinations_of_three() {
        let items = vec![1, 2, 3, 4];
        let combos3 = get_combinations(&items, 3);
        assert_eq!(combos3, vec![
            vec![1, 2, 3], vec![1, 2, 4], vec![1, 3, 4],
            vec![2, 3, 4],
        ]);
    }

    #[test]
    fn test_combinations_of_four() {
        let items = vec![1, 2, 3, 4];
        let combos4 = get_combinations(&items, 4);
        assert_eq!(combos4, vec![
            vec![1, 2, 3, 4],
        ]);
    }

    #[test]
    fn test_can_equally_divide_remainder() {
        let to_exclude_1: Group = vec![1];
        let to_exclude_2: Group = vec![2];
        let to_exclude_3: Group = vec![4];
        let list: Group = vec![1, 2, 3, 4];
        assert!(!can_equally_divide_remainder(&to_exclude_1, &list));
        assert!(can_equally_divide_remainder(&to_exclude_2, &list));
        assert!(can_equally_divide_remainder(&to_exclude_3, &list));
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
