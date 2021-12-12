use std::str::FromStr;

use common::{BadInput, InputReader};

struct Password {
    policy_min: usize,
    policy_max: usize,
    policy_char: char,
    password: String,
}

impl Password {
    fn is_valid_v1(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|c| *c == self.policy_char)
            .count();
        count >= self.policy_min && count <= self.policy_max
    }
    fn is_valid_v2(&self) -> bool {
        let chrs: Vec<char> = self.password.chars().into_iter().collect();
        (chrs[self.policy_min - 1] == self.policy_char)
            ^ (chrs[self.policy_max - 1] == self.policy_char)
    }
}

impl FromStr for Password {
    type Err = BadInput;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut splits = line.split(' ');
        let mut policy_range = splits.next().unwrap().trim().split('-');
        let policy_min = policy_range.next().unwrap().parse().unwrap();
        let policy_max = policy_range.next().unwrap().parse().unwrap();
        let policy_char = splits.next().unwrap().trim().chars().next().unwrap();
        let password = splits.next().unwrap().trim().to_string();
        Ok(Password {
            policy_min,
            policy_max,
            policy_char,
            password,
        })
    }
}

fn part1() -> usize {
    let pws: Vec<Password> = InputReader::new("input.txt").parsed_lines();
    pws.iter().filter(|p| p.is_valid_v1()).count()
}

fn part2() -> usize {
    let pws: Vec<Password> = InputReader::new("input.txt").parsed_lines();
    pws.iter().filter(|p| p.is_valid_v2()).count()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 603);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 404);
    }
}
