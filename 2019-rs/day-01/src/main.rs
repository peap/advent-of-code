use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_weights(filename: &'static str) -> Vec<i64> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| { l.unwrap().parse().unwrap() }).collect()
}

fn fuel_for_weight(weight: &i64) -> i64 {
    (weight / 3) - 2
}

fn main() {
    let weights = load_weights("input.txt");
    let total = weights.iter().fold(0, |sum, w| sum + fuel_for_weight(w));
    println!("Part 1: total weight is {}", total);
}
