use std::collections::HashMap;
use std::str::FromStr;

use common::{default_puzzle, Answer, BadInput, InputReader, Puzzle};

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
    fn get_illegal_score(&self) -> u64 {
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

    fn get_completion_score(&self) -> u64 {
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

fn part1(reader: &InputReader) -> Answer {
    let lines: Vec<NavLine> = reader.parsed_lines();
    lines.iter().fold(0, |acc, l| acc + l.get_illegal_score())
}

fn part2(reader: &InputReader) -> Answer {
    let lines: Vec<NavLine> = reader.parsed_lines();
    let mut scores: Vec<u64> = lines
        .iter()
        .map(|l| l.get_completion_score())
        .filter(|s| *s > 0)
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Syntax Scoring");
    puzzle.set_part1(part1, "total syntax error score");
    puzzle.set_part2(part2, "middle completion score");
    puzzle
}

fn main() {
    get_puzzle().run();
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
        let mut scores: Vec<u64> = lines
            .iter()
            .map(|l| l.get_completion_score())
            .filter(|s| *s > 0)
            .collect();
        scores.sort_unstable();
        assert_eq!(scores[scores.len() / 2], 288957);
    }

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(366027);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(1118645287);
    }
}
