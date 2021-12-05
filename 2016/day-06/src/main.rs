use std::collections::HashMap;

use common::InputReader;

fn get_count_maps(signals: Vec<String>) -> Vec<HashMap<char, i32>> {
    let width = signals[0].as_bytes().len();
    let mut counts: Vec<HashMap<char, i32>> = Vec::new();
    for _ in 0..width {
        counts.push(HashMap::with_capacity(26));
    }
    for sig in signals {
        for (i, chr) in sig.chars().enumerate() {
            if let Some(count) = counts[i].get_mut(&chr) {
                *count += 1;
            }
            counts[i].entry(chr).or_insert(1);
        }
    }
    counts
}

fn decode_signal_1(counts: &[HashMap<char, i32>]) -> String {
    let mut signal: Vec<char> = Vec::new();
    for count in counts {
        let mut max_num = 0;
        let mut max_char = 0 as char;
        for (chr, num) in count.iter() {
            if num > &max_num {
                max_char = *chr;
                max_num = *num;
            }
        }
        signal.push(max_char);
    }
    signal.into_iter().collect::<String>()
}

fn decode_signal_2(counts: &[HashMap<char, i32>]) -> String {
    let mut signal: Vec<char> = Vec::new();
    for count in counts {
        let mut min_num = 100;
        let mut max_char = 0 as char;
        for (chr, num) in count.iter() {
            if num < &min_num {
                max_char = *chr;
                min_num = *num;
            }
        }
        signal.push(max_char);
    }
    signal.into_iter().collect::<String>()
}

fn main() {
    let signals = InputReader::new("input.txt").string_lines();
    let count_maps = get_count_maps(signals);
    let signal_1 = decode_signal_1(&count_maps);
    let signal_2 = decode_signal_2(&count_maps);
    println!("Part 1: the signal is {}", signal_1);
    println!("Part 2: the signal is {}", signal_2);
}
