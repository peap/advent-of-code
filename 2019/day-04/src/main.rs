use std::ops::RangeInclusive;

use common::{default_puzzle, Puzzle};

const INPUT: RangeInclusive<i32> = 134564..=585159;

fn is_valid(password: i32, exact_double: bool) -> bool {
    let in_range = (100000..=999999).contains(&password);
    let mut has_repeaters = false;
    let mut has_any_exact_double = false;
    let mut repeat_count = 1;
    let mut only_increases = true;
    let mut last: i32 = 0;
    for c in format!("{}", password).chars() {
        let digit = c.to_digit(10).unwrap() as i32;
        if digit == last {
            has_repeaters = true;
            repeat_count += 1
        } else {
            if repeat_count == 2 {
                has_any_exact_double = true;
            }
            repeat_count = 1;
        }
        if digit < last {
            only_increases = false;
        }
        last = digit;
    }
    if !has_any_exact_double && repeat_count == 2 {
        has_any_exact_double = true;
    }
    in_range && has_repeaters && only_increases && (!exact_double || has_any_exact_double)
}

fn find_valid_passwords(range: RangeInclusive<i32>, exact_doubles: bool) -> Vec<i32> {
    range.filter(|p| is_valid(*p, exact_doubles)).collect()
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Secure Container");
    puzzle.set_part1("number of passwords", |_| {
        find_valid_passwords(INPUT, false).len() as u64
    });
    puzzle.set_part2("number of passwords (v2)", |_| {
        find_valid_passwords(INPUT, true).len() as u64
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
    fn test_password_validation() {
        assert!(is_valid(111111, false));
        assert!(!is_valid(223450, false));
        assert!(!is_valid(123789, false));
        assert!(is_valid(112233, true));
        assert!(!is_valid(123444, true));
        assert!(is_valid(111122, true));
    }

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(1929);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(1306);
    }
}
