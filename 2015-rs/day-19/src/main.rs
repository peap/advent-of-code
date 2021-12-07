use std::collections::HashMap;

use regex::{Captures, Regex};

use common::InputReader;

type Replacements = HashMap<String, Vec<String>>;

pub fn get_replacements_and_medicine(lines: Vec<String>) -> (Replacements, String) {
    let mut replacements: Replacements = HashMap::new();
    let mut medicine = String::new();
    let mut reading_replacements = true;
    for line in lines.iter() {
        if reading_replacements {
            if line.trim() == "" {
                reading_replacements = false;
                continue;
            }
            let mut split = line.split(" => ");
            let from = split.next().expect("Expected a first part of the split.");
            let to = split.next().expect("Expected a second part of the split.");
            if replacements.contains_key(from) {
                let tovec = replacements.get_mut(from).unwrap();
                tovec.push(to.to_string());
            } else {
                replacements.insert(from.to_string(), vec![to.to_string()]);
            }
        } else {
            medicine = line.to_string();
        }
    }
    (replacements, medicine)
}

// Translated from the Python of semi225599 and askalski.
// See: https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/cy4nsdd/
pub fn find_min_steps_reverse(from: String, to: String, replacements: &Replacements) -> u32 {
    // invert replacements
    let mut repls: HashMap<String, String> = HashMap::new();
    for (k, vs) in replacements.iter() {
        for v in vs.iter() {
            repls.insert(v.chars().rev().collect(), k.chars().rev().collect());
        }
    }
    // make replacements one-at-a-time and count
    let mut count = 0;
    let mut molecule: String = to.chars().rev().collect();
    let pattern: Vec<String> = repls.keys().map(|k| k.to_string()).collect();
    let re = Regex::new(&pattern.join("|")).expect("Could not build regex.");
    while molecule != from {
        let replaced = re.replace(&molecule, |caps: &Captures| {
            repls
                .get(caps.get(0).unwrap().as_str())
                .unwrap()
                .to_string()
        });
        molecule = replaced.into_owned();
        count += 1;
    }
    count
}

fn main() {
    let lines = InputReader::new("input.txt").parsed_lines();
    let (replacements, medicine) = get_replacements_and_medicine(lines);
    let from = "e".to_string();
    let num_steps = find_min_steps_reverse(from, medicine, &replacements);
    println!(
        "Part 2: takes {} steps to make the medicine from an electron",
        num_steps
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let lines = InputReader::new("input.txt").parsed_lines();
        let (replacements, medicine) = get_replacements_and_medicine(lines);
        let from = "e".to_string();
        let num_steps = find_min_steps_reverse(from, medicine, &replacements);
        assert_eq!(num_steps, 212);
    }
}
