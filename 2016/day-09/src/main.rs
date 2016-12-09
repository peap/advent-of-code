use std::fs::File;
use std::io::{BufReader, Read};

fn load_compressed_sequence(filename: &'static str) -> String {
    let mut sequence = String::new();
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    match reader.read_to_string(&mut sequence) {
        Ok(_) => (),
        Err(e) => panic!("Error reading {} to string: {}", filename, e),
    }
    sequence
}

fn decompress(sequence: String) -> String {
    let mut decompressed = String::with_capacity(sequence.len());
    let mut chars = sequence.chars();
    loop {
        let chr = chars.next();
        match chr {
            Some(chr) => {
                if chr == '(' {
                    let mut chr2: char;
                    let mut left_side = true;
                    let mut n_chars_str: String = String::new();
                    let mut n_times_str: String = String::new();
                    loop {
                        match chars.next() {
                            Some(c) => chr2 = c,
                            None => panic!("Unexpected end of marker."),
                        }
                        if chr2 == 'x' {
                            left_side = false;
                            continue;
                        } else if chr2 == ')' {
                            break;
                        } else {
                            if left_side {
                                n_chars_str.push(chr2);
                            } else {
                                n_times_str.push(chr2);
                            }
                        }
                    }
                    let n_chars = n_chars_str.parse::<usize>().unwrap();
                    let n_times = n_times_str.parse::<usize>().unwrap();
                    let mut seq = String::with_capacity(n_chars);
                    for _ in 0..n_chars {
                        match chars.next() {
                            Some(c) => {
                                if !c.is_whitespace() {
                                    seq.push(c);
                                }
                            }
                            None => (),
                        }
                    }
                    for _ in 0..n_times {
                        decompressed.push_str(&seq);
                    }
                } else if chr.is_whitespace() {
                    continue;
                } else {
                    decompressed.push(chr)
                }
            }
            None => {
                break;
            }
        }
    }
    decompressed
}

fn main() {
    let sequence = load_compressed_sequence("input.txt");
    let compressed_len = sequence.len();
    let expanded = decompress(sequence);
    let expanded_len = expanded.len();
    println!("Part 1: Expanded {} characters to {}.", compressed_len, expanded_len);
}

#[test]
fn test_example_1() {
    let compressed = "ADVENT".to_string();
    let expected = "ADVENT".to_string();
    assert_eq!(expected, decompress(compressed));
}

#[test]
fn test_example_2() {
    let compressed = "A(1x5)BC".to_string();
    let expected = "ABBBBBC".to_string();
    assert_eq!(expected, decompress(compressed));
}

#[test]
fn test_example_3() {
    let compressed = "(3x3)XYZ".to_string();
    let expected = "XYZXYZXYZ".to_string();
    assert_eq!(expected, decompress(compressed));
}

#[test]
fn test_example_4() {
    let compressed = "A(2x2)BCD(2x2)EFG".to_string();
    let expected = "ABCBCDEFEFG".to_string();
    assert_eq!(expected, decompress(compressed));
}

#[test]
fn test_example_5() {
    let compressed = "(6x1)(1x3)A".to_string();
    let expected = "(1x3)A".to_string();
    assert_eq!(expected, decompress(compressed));
}

#[test]
fn test_example_6() {
    let compressed = "X(8x2)(3x3)ABCY".to_string();
    let expected = "X(3x3)ABC(3x3)ABCY".to_string();
    assert_eq!(expected, decompress(compressed));
}

#[test]
fn test_whitespace_is_skipped() {
    let compressed = "X(8x2)(3x3)ABCY Z".to_string();
    let expected = "X(3x3)ABC(3x3)ABCYZ".to_string();
    assert_eq!(expected, decompress(compressed));
}

#[test]
fn test_whitespace_is_skipped_in_marker_sequence() {
    let compressed = "X(8x2)(3x3)A BCY".to_string();
    let expected = "X(3x3)AB(3x3)ABCY".to_string();
    assert_eq!(expected, decompress(compressed));
}

#[test]
fn test_part1_answer() {
    let sequence = load_compressed_sequence("input.txt");
    let expanded = decompress(sequence);
    assert_eq!(expanded.len(), 120765);
}
