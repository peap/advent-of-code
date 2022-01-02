use common::{default_puzzle, Puzzle};

fn find_two_nums_with_sum(nums: Vec<u64>, sum: u64) -> Vec<u64> {
    for (i, first) in nums.iter().enumerate() {
        for second in nums[i + 1..nums.len()].iter() {
            if first + second == sum {
                return vec![*first, *second];
            }
        }
    }
    vec![0, 0]
}

fn find_three_nums_with_sum(nums: Vec<u64>, sum: u64) -> Vec<u64> {
    for (i, first) in nums.iter().enumerate() {
        for (j, second) in nums[i + 1..nums.len()].iter().enumerate() {
            for third in nums[j + 1..nums.len()].iter() {
                if first + second + third == sum {
                    return vec![*first, *second, *third];
                }
            }
        }
    }
    vec![0, 0]
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Report Repair");
    puzzle.set_part1("product of two entries that sum to 2020", |reader| {
        let expenses = reader.parsed_lines();
        let nums = find_two_nums_with_sum(expenses, 2020);
        nums[0] * nums[1]
    });
    puzzle.set_part2("product of three entries that sum to 2020", |reader| {
        let expenses = reader.parsed_lines();
        let nums3 = find_three_nums_with_sum(expenses, 2020);
        nums3[0] * nums3[1] * nums3[2]
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
        get_puzzle().test_part1(247 * 1773);
        // let expenses = InputReader::new("input.txt").parsed_lines();
        // let nums = find_two_nums_with_sum(expenses, 2020);
        // assert_eq!(nums[0], 247);
        // assert_eq!(nums[1], 1773);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(188 * 936 * 896);
        // let expenses = InputReader::new("input.txt").parsed_lines();
        // let nums = find_three_nums_with_sum(expenses, 2020);
        // assert_eq!(nums[0], 188);
        // assert_eq!(nums[1], 936);
        // assert_eq!(nums[2], 896);
    }
}
