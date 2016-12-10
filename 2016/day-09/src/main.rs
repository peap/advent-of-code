use std::fs::File;
use std::io::{BufReader, Read};
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Mode {
    BuildMode,
    CountMode,
}

#[derive(Debug, PartialEq)]
pub enum Decompressed {
    Built(String),
    Counted(usize),
}

impl Decompressed {
    fn len(&self) -> usize {
        match *self {
            Decompressed::Built(ref s) => s.len(),
            Decompressed::Counted(n) => n,
        }
    }
}

pub fn load_compressed_sequence(filename: &'static str) -> String {
    let mut sequence = String::new();
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    match reader.read_to_string(&mut sequence) {
        Ok(_) => (),
        Err(e) => panic!("Error reading {} to string: {}", filename, e),
    }
    sequence
}

pub fn parse_marker(chars: &mut Chars) -> (usize, usize) {
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
    (n_chars, n_times)
}

pub fn decompress<'a>(sequence: &'a str,
                  mode: Mode,
                  recursive: bool) -> Decompressed {
    use Mode::*;
    use Decompressed::*;

    let mut chars = sequence.chars();
    let mut decompressed = String::with_capacity(sequence.len());
    let mut count = 0;
    let mut fully_expanded = true;

    {
        // introduced in this inner scope so that the mutable borrow in add_char
        // goes out of scope
        let mut add_char = |c| {
            match mode {
                BuildMode => decompressed.push(c),
                CountMode => count += 1,
            }
        };

        loop {
            let chr = chars.next();
            match chr {
                Some(chr) => {
                    if chr == '(' {
                        let (n_chars, n_times) = parse_marker(&mut chars);
                        let mut seq = String::with_capacity(n_chars);
                        for _ in 0..n_chars {
                            match chars.next() {
                                Some(c) => {
                                    if c == '(' {
                                        fully_expanded = false;
                                        if recursive && mode == CountMode {
                                        }
                                    }
                                    if !c.is_whitespace() {
                                        seq.push(c);
                                    }
                                }
                                None => (),
                            }
                        }
                        for _ in 0..n_times {
                            for c in seq.chars() {
                                add_char(c);
                            }
                        }
                    } else if chr.is_whitespace() {
                        continue;
                    } else {
                        add_char(chr);
                    }
                }
                None => {
                    break;
                }
            }
        }
    }
    if recursive && !fully_expanded {
        match mode {
            BuildMode => decompress(&decompressed, mode, recursive),
            CountMode => Counted(count),  // recursion avoided
        }
    } else {
        match mode {
            BuildMode => Built(decompressed),
            CountMode => Counted(count),
        }
    }
}

fn main() {
    use Mode::*;
    let sequence = load_compressed_sequence("input.txt");
    let compressed_len = &sequence.len();
    let expanded_1 = decompress(&sequence, BuildMode, false);
    let len_1 = expanded_1.len();
    println!("Part 1: Expanded {} characters to {}.", compressed_len, len_1);
    let expanded_2 = decompress(&sequence, CountMode, true);
    let len_2 = expanded_2.len();
    println!("Part 2: Expanded {} characters to {}.", compressed_len, len_2);
}

#[cfg(test)]
mod tests {

    use super::*;
    use super::Decompressed::*;
    use super::Mode::*;

    // Example 1

    #[test]
    fn test_example_1() {
        let compressed = "ADVENT";
        let expected = Built("ADVENT".to_string());
        assert_eq!(expected,decompress(compressed, BuildMode, false));
    }

    #[test]
    fn test_example_1_count() {
        let compressed = "ADVENT";
        let expected = Counted("ADVENT".len());
        assert_eq!(expected, decompress(compressed, CountMode, false));
    }

    #[test]
    fn test_example_1_recursive() {
        let compressed = "ADVENT";
        let expected = Built("ADVENT".to_string());
        assert_eq!(expected,decompress(compressed, BuildMode, true));
    }

    #[test]
    fn test_example_1_count_recursive() {
        let compressed = "ADVENT";
        let expected = Counted("ADVENT".len());
        assert_eq!(expected, decompress(compressed, CountMode, true));
    }

    // Example 2

    #[test]
    fn test_example_2() {
        let compressed = "A(1x5)BC";
        let expected = Built("ABBBBBC".to_string());
        assert_eq!(expected, decompress(compressed, BuildMode, false));
    }

    #[test]
    fn test_example_2_count() {
        let compressed = "A(1x5)BC";
        let expected = Counted("ABBBBBC".len());
        assert_eq!(expected, decompress(compressed, CountMode, false));
    }

    #[test]
    fn test_example_2_recursive() {
        let compressed = "A(1x5)BC";
        let expected = Built("ABBBBBC".to_string());
        assert_eq!(expected, decompress(compressed, BuildMode, true));
    }

    #[test]
    fn test_example_2_count_recursive() {
        let compressed = "A(1x5)BC";
        let expected = Counted("ABBBBBC".len());
        assert_eq!(expected, decompress(compressed, CountMode, true));
    }

    // Example 3

    #[test]
    fn test_example_3() {
        let compressed = "(3x3)XYZ";
        let expected = Built("XYZXYZXYZ".to_string());
        assert_eq!(expected, decompress(compressed, BuildMode, false));
    }

    #[test]
    fn test_example_3_count() {
        let compressed = "(3x3)XYZ";
        let expected = Counted("XYZXYZXYZ".len());
        assert_eq!(expected, decompress(compressed, CountMode, false));
    }

    #[test]
    fn test_example_3_recursive() {
        let compressed = "(3x3)XYZ";
        let expected = Built("XYZXYZXYZ".to_string());
        assert_eq!(expected, decompress(compressed, BuildMode, true));
    }

    #[test]
    fn test_example_3_count_recursive() {
        let compressed = "(3x3)XYZ";
        let expected = Counted("XYZXYZXYZ".len());
        assert_eq!(expected, decompress(compressed, CountMode, true));
    }

    // Example 4

    #[test]
    fn test_example_4() {
        let compressed = "A(2x2)BCD(2x2)EFG";
        let expected = Built("ABCBCDEFEFG".to_string());
        assert_eq!(expected, decompress(compressed, BuildMode, false));
    }

    #[test]
    fn test_example_4_count() {
        let compressed = "A(2x2)BCD(2x2)EFG";
        let expected = Counted("ABCBCDEFEFG".len());
        assert_eq!(expected, decompress(compressed, CountMode, false));
    }

    #[test]
    fn test_example_4_recursive() {
        let compressed = "A(2x2)BCD(2x2)EFG";
        let expected = Built("ABCBCDEFEFG".to_string());
        assert_eq!(expected, decompress(compressed, BuildMode, true));
    }

    #[test]
    fn test_example_4_count_recursive() {
        let compressed = "A(2x2)BCD(2x2)EFG";
        let expected = Counted("ABCBCDEFEFG".len());
        assert_eq!(expected, decompress(compressed, CountMode, true));
    }

    // Example 5

    #[test]
    fn test_example_5() {
        let compressed = "(6x1)(1x3)A";
        let expected = Built("(1x3)A".to_string());
        assert_eq!(expected, decompress(compressed, BuildMode, false));
    }

    #[test]
    fn test_example_5_count() {
        let compressed = "(6x1)(1x3)A";
        let expected = Counted("(1x3)A".len());
        assert_eq!(expected, decompress(compressed, CountMode, false));
    }

    #[test]
    fn test_example_5_recursive() {
        let compressed = "(6x1)(1x3)A";
        let expected = Built("AAA".to_string());
        assert_eq!(expected, decompress(compressed, BuildMode, true));
    }

    #[test]
    fn test_example_5_count_recursive() {
        let compressed = "(6x1)(1x3)A";
        let expected = Counted("AAA".len());
        assert_eq!(expected, decompress(compressed, CountMode, true));
    }

    // Example 6

    #[test]
    fn test_example_6() {
        let compressed = "X(8x2)(3x3)ABCY";
        let expected = Built("X(3x3)ABC(3x3)ABCY".to_string());
        assert_eq!(expected, decompress(compressed, BuildMode, false));
    }

    #[test]
    fn test_example_6_count() {
        let compressed = "X(8x2)(3x3)ABCY";
        let expected = Counted("X(3x3)ABC(3x3)ABCY".len());
        assert_eq!(expected, decompress(compressed, CountMode, false));
    }

    // Whitespace is skipped outside marker sequence

    #[test]
    fn test_whitespace_is_skipped() {
        let compressed = "X(8x2)(3x3)ABCY Z";
        let expected = Built("X(3x3)ABC(3x3)ABCYZ".to_string());
        assert_eq!(expected, decompress(compressed, BuildMode, false));
    }

    #[test]
    fn test_whitespace_is_skipped_count() {
        let compressed = "X(8x2)(3x3)ABCY Z";
        let expected = Counted("X(3x3)ABC(3x3)ABCYZ".len());
        assert_eq!(expected, decompress(compressed, CountMode, false));
    }

    // Whitespace skipped in marker sequence (expectations are mine)

    #[test]
    fn test_whitespace_is_skipped_in_marker_sequence() {
        let compressed = "X(8x2)(3x3)A BCY";
        let expected = Built("X(3x3)AB(3x3)ABCY".to_string());
        assert_eq!(expected, decompress(compressed, BuildMode, false));
    }

    #[test]
    fn test_whitespace_is_skipped_in_marker_sequence_count() {
        let compressed = "X(8x2)(3x3)A BCY";
        let expected = Counted("X(3x3)AB(3x3)ABCY".len());
        assert_eq!(expected, decompress(compressed, CountMode, false));
    }

    // Part 1

    #[test]
    fn test_part1_answer() {
        let sequence = load_compressed_sequence("input.txt");
        let expanded = decompress(&sequence, BuildMode, false);
        assert_eq!(expanded.len(), 120765);
    }

    #[test]
    fn test_part1_answer_count() {
        let sequence = load_compressed_sequence("input.txt");
        let expanded = decompress(&sequence, CountMode, false);
        assert_eq!(expanded.len(), 120765);
    }

    // Part 2

    #[test]
    fn test_part2_example_1() {
        let compressed = "(3x3)XYZ";
        let expected = Built("XYZXYZXYZ".to_string());
        let expected_count = Counted("XYZXYZXYZ".len());
        assert_eq!(expected, decompress(compressed, BuildMode, true));
        assert_eq!(expected_count, decompress(compressed, CountMode, true));
    }

    #[test]
    fn test_part2_example_2() {
        let compressed = "X(8x2)(3x3)ABCY";
        let expected = Built("XABCABCABCABCABCABCY".to_string());
        let expected_count = Counted("XABCABCABCABCABCABCY".len());
        assert_eq!(expected, decompress(compressed, BuildMode, true));
        assert_eq!(expected_count, decompress(compressed, CountMode, true));
    }

    #[test]
    fn test_part2_example_3() {
        let compressed = "(27x12)(20x12)(13x14)(7x10)(1x12)A";
        let expected_count = Counted(241920);
        assert_eq!(expected_count, decompress(compressed, CountMode, true));
    }

    #[test]
    fn test_part2_example_4() {
        let compressed = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN";
        let expected_count = Counted(445);
        assert_eq!(expected_count, decompress(compressed, CountMode, true));
    }

}
