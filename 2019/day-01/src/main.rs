use common::InputReader;
use std::cmp;

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

fn main() {
    let weights = InputReader::new("input.txt").i64_lines();
    let total1 = weights.iter().fold(0, |sum, w| sum + fuel_for_weight(w));
    println!("Part 1: total weight is {}", total1);
    let total2 = weights
        .iter()
        .fold(0, |sum, w| sum + fuel_for_weight_recur(w));
    println!("Part 2: total weight (w/fuel weight) is {}", total2);
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
    fn test_part1() {
        let weights = InputReader::new("input.txt").i64_lines();
        let total1 = weights.iter().fold(0, |sum, w| sum + fuel_for_weight(w));
        assert_eq!(total1, 3324332);
    }

    #[test]
    fn test_fuel_for_weight_recur() {
        assert_eq!(fuel_for_weight_recur(&14), 2);
        assert_eq!(fuel_for_weight_recur(&1969), 966);
        assert_eq!(fuel_for_weight_recur(&100756), 50346);
    }

    #[test]
    fn test_part2() {
        let weights = InputReader::new("input.txt").i64_lines();
        let total2 = weights
            .iter()
            .fold(0, |sum, w| sum + fuel_for_weight_recur(w));
        assert_eq!(total2, 4983626);
    }
}
