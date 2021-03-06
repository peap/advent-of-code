use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_signals(filename: &'static str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut signals = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(signal) => signals.push(signal),
            Err(_) => ()
        }
    }
    signals
}

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
            if !counts[i].contains_key(&chr) {
                counts[i].insert(chr, 1);
            }
        }
    }
    counts
}

fn decode_signal_1(counts: &Vec<HashMap<char, i32>>) -> String {
    let mut signal: Vec<char> = Vec::new();
    for count in counts {
        let mut max_num = 0;
        let mut max_char = 0 as char;
        for (chr, num) in count.iter() {
            if num > &max_num {
                max_char = chr.clone();
                max_num = num.clone();
            }
        }
        signal.push(max_char);
    }
    signal.into_iter().collect::<String>()
}

fn decode_signal_2(counts: &Vec<HashMap<char, i32>>) -> String {
    let mut signal: Vec<char> = Vec::new();
    for count in counts {
        let mut min_num = 100;
        let mut max_char = 0 as char;
        for (chr, num) in count.iter() {
            if num < &min_num {
                max_char = chr.clone();
                min_num = num.clone();
            }
        }
        signal.push(max_char);
    }
    signal.into_iter().collect::<String>()
}

fn main() {
    let signals = load_signals("input.txt");
    let count_maps = get_count_maps(signals);
    let signal_1 = decode_signal_1(&count_maps);
    let signal_2 = decode_signal_2(&count_maps);
    println!("Part 1: the signal is {}", signal_1);
    println!("Part 2: the signal is {}", signal_2);
}
