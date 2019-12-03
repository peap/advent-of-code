use std::fs::File;
use std::io::Read;

fn load_input(filename: &'static str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.trim().to_string()
}

fn captcha(input: String) -> i64 {
    let digits: Vec<u32> = input.chars().map(|c| {
        c.to_digit(10).unwrap()
    }).collect();
    let mut sum = 0;
    let len = digits.len();
    for i in 0..len {
        let this = digits[i] as i64;
        let next = digits[(i + 1) % len] as i64;
        if this == next {
            sum += this;
        }
    }
    sum
}

fn main() {
    let input = load_input("input.txt");
    println!("Part 1: Captcha for input is {}", captcha(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_captcha_examples() {
        let examples: Vec<(String, i64)> = vec![
            (String::from("1122"), 3),
            (String::from("1111"), 4),
            (String::from("1234"), 0),
            (String::from("91212129"), 9),
        ];
        for (input, expected) in examples {
            assert_eq!(captcha(input), expected);
        }
    }

    #[test]
    fn test_part1() {
        let input = load_input("input.txt");
        assert_eq!(captcha(input), 1136);
    }
}
