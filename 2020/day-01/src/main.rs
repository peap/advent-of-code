use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_expenses(filename: &'static str) -> Vec<i64> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}

fn find_two_nums_with_sum(nums: Vec<i64>, sum: i64) -> Vec<i64> {
    for (i, first) in nums.iter().enumerate() {
        for second in nums[i+1..nums.len()].iter() {
            if first + second == sum {
                return vec![first.clone(), second.clone()];
            }
        }
    }
    vec![0, 0]
}

fn find_three_nums_with_sum(nums: Vec<i64>, sum: i64) -> Vec<i64> {
    for (i, first) in nums.iter().enumerate() {
        for (j, second) in nums[i+1..nums.len()].iter().enumerate() {
            for third in nums[j+1..nums.len()].iter() {
                if first + second + third == sum {
                    return vec![first.clone(), second.clone(), third.clone()];
                }
            }
        }
    }
    vec![0, 0]
}

fn main() {
    let expenses = load_expenses("input.txt");
    let nums2 = find_two_nums_with_sum(expenses.clone(), 2020);
    let product = nums2[0] * nums2[1];
    println!("Part 1: {} * {} = {}", nums2[0], nums2[1], product);
    let nums3 = find_three_nums_with_sum(expenses.clone(), 2020);
    let product = nums3[0] * nums3[1] * nums3[2];
    println!("Part 2: {} * {} * {} = {}", nums3[0], nums3[1], nums3[2], product);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_two_nums_with_sum() {
        let ex1 = vec![1721, 979, 366, 299, 675, 1456];
        let nums = find_two_nums_with_sum(ex1, 2020);
        assert_eq!(nums[0], 1721);
        assert_eq!(nums[1], 299);
    }

    #[test]
    fn test_find_three_nums_with_sum() {
        let ex1 = vec![1721, 979, 366, 299, 675, 1456];
        let nums = find_three_nums_with_sum(ex1, 2020);
        assert_eq!(nums[0], 979);
        assert_eq!(nums[1], 366);
        assert_eq!(nums[2], 675);
    }

    #[test]
    fn test_part1() {
        let expenses = load_expenses("input.txt");
        let nums = find_two_nums_with_sum(expenses, 2020);
        assert_eq!(nums[0], 247);
        assert_eq!(nums[1], 1773);
    }

    #[test]
    fn test_part2() {
        let expenses = load_expenses("input.txt");
        let nums = find_three_nums_with_sum(expenses, 2020);
        assert_eq!(nums[0], 188);
        assert_eq!(nums[1], 936);
        assert_eq!(nums[2], 896);
    }
}
