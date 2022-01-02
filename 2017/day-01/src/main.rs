use common::{default_puzzle, Puzzle};

fn captcha(digits: Vec<u8>, distance: usize) -> u64 {
    let mut sum = 0;
    let len = digits.len();
    for i in 0..len {
        let this = digits[i];
        let next = digits[(i + distance) % len];
        if this == next {
            sum += this as u64;
        }
    }
    sum
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Inverse Captcha");
    puzzle.set_part1("captcha for input", |reader| {
        let digits = reader.digit_line(10);
        captcha(digits, 1)
    });
    puzzle.set_part2("captcha v2 for input", |reader| {
        let digits = reader.digit_line(10);
        let distance = digits.len() / 2;
        captcha(digits, distance)
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
    fn test_captcha_examples() {
        let examples: Vec<(&'static str, usize, u64)> = vec![
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
        get_puzzle().test_part1(1136);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(1092);
    }
}
