use std::collections::HashMap;
use std::str::FromStr;

use common::{BadInput, InputReader};

struct NavLine {
    line: String,
}

impl FromStr for NavLine {
    type Err = BadInput;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(NavLine {
            line: line.to_string(),
        })
    }
}

impl NavLine {
    fn get_illegal_score(&self) -> i64 {
        let pairs: HashMap<char, char> =
            HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
        let mut delims: Vec<char> = vec![self.line.chars().next().unwrap()];
        for c in self.line.chars().skip(1) {
            match c {
                '(' | '[' | '{' | '<' => delims.push(c),
                ')' | ']' | '}' | '>' => {
                    let last = delims.pop().unwrap();
                    if let Some(want) = pairs.get(&last) {
                        if c == *want {
                            continue;
                        } else {
                            return match c {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' => 25137,
                                _ => 0,
                            };
                        }
                    }
                }
                _ => panic!("Unknonwn char {}", c),
            }
        }
        0
    }

    fn get_completion_score(&self) -> i64 {
        let pairs: HashMap<char, char> =
            HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
        let mut delims: Vec<char> = vec![self.line.chars().next().unwrap()];
        for c in self.line.chars().skip(1) {
            match c {
                '(' | '[' | '{' | '<' => delims.push(c),
                ')' | ']' | '}' | '>' => {
                    let last = delims.pop().unwrap();
                    if let Some(want) = pairs.get(&last) {
                        if c == *want {
                            continue;
                        } else {
                            // illegal line
                            return 0;
                        }
                    }
                }
                _ => panic!("Unknonwn char {}", c),
            }
        }
        delims.iter().rev().fold(0, |acc, d| {
            5 * acc
                + match pairs.get(d).unwrap() {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                }
        })
    }
}

fn part1() -> i64 {
    let lines: Vec<NavLine> = InputReader::new("input.txt").parsed_lines();
    lines.iter().fold(0, |acc, l| acc + l.get_illegal_score())
}

fn part2() -> i64 {
    let lines: Vec<NavLine> = InputReader::new("input.txt").parsed_lines();
    let mut scores: Vec<i64> = lines
        .iter()
        .map(|l| l.get_completion_score())
        .filter(|s| *s > 0)
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let lines: Vec<NavLine> = vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ]
        .iter()
        .map(|l| l.parse().unwrap())
        .collect();
        assert_eq!(
            lines.iter().fold(0, |acc, l| acc + l.get_illegal_score()),
            26397
        );
        let mut scores: Vec<i64> = lines
            .iter()
            .map(|l| l.get_completion_score())
            .filter(|s| *s > 0)
            .collect();
        scores.sort_unstable();
        assert_eq!(scores[scores.len() / 2], 288957);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 366027);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1118645287);
    }
}
