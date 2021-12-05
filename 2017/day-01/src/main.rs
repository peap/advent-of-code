use std::fs::File;
use std::io::Read;

fn load_input(filename: &'static str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.trim().to_string()
}

fn captcha(input: &str, distance: usize) -> i64 {
    let digits: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
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
    let input = load_input("input.txt");
    println!("Part 1: Captcha for input is {}", captcha(&input, 1));
    let distance = &input.chars().count() / 2;
    println!(
        "Part 2: Captcha v2 for input is {}",
        captcha(&input, distance)
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
            assert_eq!(captcha(input, distance), expected);
        }
    }

    #[test]
    fn test_part1() {
        let input = load_input("input.txt");
        assert_eq!(captcha(&input, 1), 1136);
    }

    #[test]
    fn test_part2() {
        let input = load_input("input.txt");
        let distance = &input.chars().count() / 2;
        assert_eq!(captcha(&input, distance), 1092);
    }
}
