use std::ops::RangeInclusive;

const INPUT: RangeInclusive<i32> = 134564..=585159;

fn is_valid(password: i32) -> bool {
    let in_range = password >= 100000 && password <= 999999;
    let mut has_repeaters = false;
    let mut only_increases = true;
    let mut last = 0;
    for c in format!("{}", password).chars() {
        let digit = c.to_digit(10).unwrap();
        if digit == last {
            has_repeaters = true;
        }
        if digit < last {
            only_increases = false;
        }
        last = digit;
    }
    in_range && has_repeaters && only_increases
}

fn find_valid_passwords(range: RangeInclusive<i32>) -> Vec<i32> {
    range.filter(|p| is_valid(*p)).collect()
}

fn part1() -> i32 {
    find_valid_passwords(INPUT).len() as i32
}

fn main() {
    println!("Part 1: Number of passwords found: {}", part1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_validation() {
        assert!(is_valid(111111));
        assert!(!is_valid(223450));
        assert!(!is_valid(123789));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1929);
    }

}
