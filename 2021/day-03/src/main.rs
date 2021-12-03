use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_report(filename: &'static str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap())
        .collect()
}

fn sum_digits(numbers: &Vec<String>) -> Vec<u32> {
    let mut sums = vec![0; numbers[0].len()];
    for num in numbers.iter() {
        for (i, c) in num.chars().enumerate() {
            let digit = c.to_digit(10).unwrap();
            sums[i] += digit;
        }
    }
    sums
}

fn gamma_epsilon(numbers: Vec<String>) -> (u32, u32) {
    let sums = sum_digits(&numbers);
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for s in sums.iter() {
        if *s as usize > numbers.len() / 2 {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
    }
    (u32::from_str_radix(&gamma, 2).unwrap(),
     u32::from_str_radix(&epsilon, 2).unwrap())
}

fn gas_rating(numbers: Vec<String>, most_common: bool) -> u32 {
    let mut remaining = numbers.clone();
    let mut index = 0;
    loop {
        let count = remaining.len() as u32;
        let sums = sum_digits(&remaining);
        let mid: f64 = count as f64 / 2.0;
        let mut winner = if sums[index] as f64 > mid { '1' } else { '0' };
        if !most_common {
            winner = if winner == '1' { '0' } else { '1' };
        }
        if count % 2 == 0 && sums[index] == count / 2 {
            winner = if most_common { '1' } else { '0' };
        }
        remaining.retain(|n| n.chars().nth(index).unwrap() == winner);
        if remaining.len() == 1 {
            println!("{:?}", remaining);
            break;
        } else if remaining.len() == 0 {
            panic!("oops");
        }
        index += 1;
    }
    u32::from_str_radix(&remaining[0], 2).unwrap()
}

fn main() {
    let numbers = load_report("input.txt");
    let (g, e) = gamma_epsilon(numbers.clone());
    println!("Part 1: gamma: {}, epsilon: {}; g * e = {}", g, e, g * e);
    let o2 = gas_rating(numbers.clone(), true);
    let co2 = gas_rating(numbers.clone(), false);
    println!("Part 2: oxygen: {}, co2: {}; product = {}", o2, co2, o2 * co2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let ex1: Vec<String> = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111",
            "11100", "10000", "11001", "00010", "01010",
        ].iter().map(|&s| s.into()).collect();
        assert_eq!(gamma_epsilon(ex1.clone()), (22, 9));
        assert_eq!(gas_rating(ex1.clone(), true), 23);
        assert_eq!(gas_rating(ex1.clone(), false), 10);
    }

    #[test]
    fn test_part1() {
        let numbers = load_report("input.txt");
        assert_eq!(gamma_epsilon(numbers), (3529, 566));
    }

    #[test]
    fn test_part2() {
        let numbers = load_report("input.txt");
        assert_eq!(gas_rating(numbers.clone(), true), 3573);
        assert_eq!(gas_rating(numbers.clone(), false), 289);
    }
}
