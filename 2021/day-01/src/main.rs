use common::InputReader;

fn count_increases(nums: Vec<i64>, window: usize) -> i64 {
    let mut count = 0;
    let mut slider = vec![];
    slider.extend_from_slice(&nums[0..window]);
    let mut last: i64 = slider.iter().sum();
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

fn main() {
    let reader = InputReader::new("input.txt");
    let depths = reader.i64_lines();
    println!(
        "Part 1: depth increases: {}",
        count_increases(depths.clone(), 1)
    );
    println!("Part 2: depth increases: {}", count_increases(depths, 3));
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
        let reader = InputReader::new("input.txt");
        let depths = reader.i64_lines();
        assert_eq!(count_increases(depths, 1), 1722);
    }

    #[test]
    fn test_part2() {
        let reader = InputReader::new("input.txt");
        let depths = reader.i64_lines();
        assert_eq!(count_increases(depths, 3), 1748);
    }
}
