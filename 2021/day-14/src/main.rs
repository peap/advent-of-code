use std::cmp::{max, min};
use std::collections::HashMap;

use common::{default_puzzle, Answer, InputReader, Puzzle};

struct Polymer {
    template: String,
    rules: HashMap<(char, char), char>,
    n_insertions: usize,
}

impl Polymer {
    fn new(lines: Vec<String>, n: usize) -> Self {
        let mut rules: HashMap<(char, char), char> = HashMap::new();
        for l in lines[2..].iter() {
            let pair1 = l.chars().next().unwrap();
            let pair2 = l.chars().nth(1).unwrap();
            let insertion = l.chars().last().unwrap();
            rules.insert((pair1, pair2), insertion);
        }
        Polymer {
            template: lines[0].clone(),
            rules,
            n_insertions: n,
        }
    }

    fn expand_and_count(&self) -> HashMap<char, u64> {
        let mut char_counts: HashMap<char, u64> = HashMap::new();
        let mut pair_counts: HashMap<(char, char), u64> = HashMap::new();
        let mut last = self.template.chars().next().unwrap();
        self.template.chars().skip(1).for_each(|ch| {
            *char_counts.entry(ch).or_insert(0) += 1;
            *pair_counts.entry((last, ch)).or_insert(0) += 1;
            last = ch;
        });
        for _ in 0..self.n_insertions {
            let these_pairs = pair_counts.clone();
            for (pair, count) in these_pairs.into_iter() {
                if let Some(ch) = self.rules.get(&pair) {
                    // Add the new characters.
                    *char_counts.entry(*ch).or_insert(0) += count;
                    // Remove the old pair and add the two new ones.
                    *pair_counts.entry(pair).or_insert(0) -= count;
                    *pair_counts.entry((pair.0, *ch)).or_insert(0) += count;
                    *pair_counts.entry((*ch, pair.1)).or_insert(0) += count;
                }
            }
        }
        char_counts
    }

    fn most_minus_least(&mut self) -> u64 {
        let char_counts = self.expand_and_count();
        let (mut low, mut high) = (u64::MAX, 0);
        for (_, count) in char_counts.into_iter() {
            low = min(low, count);
            high = max(high, count);
        }
        high - low
    }
}

fn part1(reader: &InputReader) -> Answer {
    let lines: Vec<String> = reader.parsed_lines();
    Polymer::new(lines, 10).most_minus_least()
}

fn part2(reader: &InputReader) -> Answer {
    let lines: Vec<String> = reader.parsed_lines();
    Polymer::new(lines, 40).most_minus_least()
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Extended Polymerization");
    puzzle.set_part1(part1, "most common minus least common (n=10)");
    puzzle.set_part2(part2, "most common minus least common (n=40)");
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
        let lines: Vec<String> = vec![
            "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B",
            "HN -> C", "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B",
            "CC -> N", "CN -> C",
        ]
        .iter()
        .map(|l| l.to_string())
        .collect();
        let mut polymer = Polymer::new(lines.clone(), 10);
        assert_eq!(polymer.most_minus_least(), 1588);
        let mut polymer = Polymer::new(lines, 40);
        assert_eq!(polymer.most_minus_least(), 2188189693529);
    }

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(2345);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(2432786807053);
    }
}
