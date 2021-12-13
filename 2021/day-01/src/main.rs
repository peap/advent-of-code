use common::{default_puzzle, Answer, InputReader, Puzzle};

fn count_increases(nums: Vec<u64>, window: usize) -> u64 {
    let mut count = 0;
    let mut slider = vec![];
    slider.extend_from_slice(&nums[0..window]);
    let mut last: u64 = slider.iter().sum();
    for n in nums[window..nums.len()].iter() {
        slider.rotate_left(1);
        slider[window - 1] = *n;
        let new_sum = slider.iter().sum();
        if new_sum > last {
            count += 1;
        }
        last = new_sum;
    }
    count
}

fn part1(reader: &InputReader) -> Answer {
    let depths = reader.parsed_lines();
    count_increases(depths, 1)
}

fn part2(reader: &InputReader) -> Answer {
    let depths = reader.parsed_lines();
    count_increases(depths, 3)
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Sonar Sweep");
    puzzle.set_part1(part1, "depth increases (w=1)");
    puzzle.set_part2(part2, "depth increases (w=3)");
    puzzle
}

fn main() {
    get_puzzle().run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_increases() {
        let ex1 = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increases(ex1.clone(), 1), 7);
        assert_eq!(count_increases(ex1, 3), 5);
    }

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(1722);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(1748);
    }
}
