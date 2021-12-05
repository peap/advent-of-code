use common::InputReader;

fn captcha(digits: Vec<u8>, distance: usize) -> i64 {
    let mut sum = 0;
    let len = digits.len();
    for i in 0..len {
        let this = digits[i];
        let next = digits[(i + distance) % len];
        if this == next {
            sum += this as i64;
        }
    }
    sum
}

fn main() {
    let digits = InputReader::new("input.txt").u8_line();
    println!(
        "Part 1: Captcha for input is {}",
        captcha(digits.clone(), 1)
    );
    let distance = digits.len() / 2;
    println!(
        "Part 2: Captcha v2 for input is {}",
        captcha(digits, distance)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_captcha_examples() {
        let examples: Vec<(&'static str, usize, i64)> = vec![
            // Part 1 examples
            ("1122", 1, 3),
            ("1111", 1, 4),
            ("1234", 1, 0),
            ("91212129", 1, 9),
            // Part 2 examples
            ("1212", 2, 6),
            ("1221", 2, 0),
            ("123425", 3, 4),
            ("123123", 3, 12),
            ("12131415", 4, 4),
        ];
        for (input, distance, expected) in examples {
            let digits = input
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect();
            assert_eq!(captcha(digits, distance), expected);
        }
    }

    #[test]
    fn test_part1() {
        let digits = InputReader::new("input.txt").u8_line();
        assert_eq!(captcha(digits, 1), 1136);
    }

    #[test]
    fn test_part2() {
        let digits = InputReader::new("input.txt").u8_line();
        let distance = digits.len() / 2;
        assert_eq!(captcha(digits, distance), 1092);
    }
}
