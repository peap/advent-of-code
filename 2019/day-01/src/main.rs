use std::cmp;

use common::{default_puzzle, Answer, InputReader, Puzzle};

fn fuel_for_weight(weight: &i64) -> i64 {
    cmp::max(0, (weight / 3) - 2)
}

fn fuel_for_weight_recur(weight: &i64) -> i64 {
    let fuel = fuel_for_weight(weight);
    if fuel == 0 {
        return fuel;
    }
    fuel + fuel_for_weight_recur(&fuel)
}

fn part1(reader: &InputReader) -> Answer {
    let weights = reader.parsed_lines();
    weights
        .iter()
        .fold(0, |sum, w| sum + fuel_for_weight(w) as u64)
}

fn part2(reader: &InputReader) -> Answer {
    let weights = reader.parsed_lines();
    weights
        .iter()
        .fold(0, |sum, w| sum + fuel_for_weight_recur(w) as u64)
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("The Tyranny of the Rocket Equation");
    puzzle.set_part1(part1, "total weight");
    puzzle.set_part2(part2, "total weight (w/fuel)");
    puzzle
}

fn main() {
    get_puzzle().run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_for_weight() {
        assert_eq!(fuel_for_weight(&1), 0);
        assert_eq!(fuel_for_weight(&5), 0);
        assert_eq!(fuel_for_weight(&12), 2);
        assert_eq!(fuel_for_weight(&14), 2);
        assert_eq!(fuel_for_weight(&1969), 654);
        assert_eq!(fuel_for_weight(&100756), 33583);
    }

    #[test]
    fn test_fuel_for_weight_recur() {
        assert_eq!(fuel_for_weight_recur(&14), 2);
        assert_eq!(fuel_for_weight_recur(&1969), 966);
        assert_eq!(fuel_for_weight_recur(&100756), 50346);
    }

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(3324332);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(4983626);
    }
}
