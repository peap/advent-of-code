use std::ops::RangeInclusive;

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

fn part1() -> i32 {
    find_valid_passwords(INPUT, false).len() as i32
}

fn part2() -> i32 {
    find_valid_passwords(INPUT, true).len() as i32
}

fn main() {
    println!("Part 1: Number of passwords found: {}", part1());
    println!("Part 2: Number of passwords found: {}", part2());
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
        assert_eq!(part1(), 1929);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1306);
    }
}
