use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug)]
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

pub fn decompress<'a>(sequence: &'a str,
                  mode: Mode,
                  recursize: bool) -> Decompressed {
    use Mode::*;

    let mut chars = sequence.chars();
    let mut decompressed = String::with_capacity(sequence.len());
    let mut count = 0;

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
                            for c in seq.chars() {
                                add_char(c);
                            }
                            // decompressed.push_str(&seq);
                        }
                    } else if chr.is_whitespace() {
                        continue;
                    } else {
                        add_char(chr);
                        // decompressed.push(chr)
                    }
                }
                None => {
                    break;
                }
            }
        }
    }
    match mode {
        BuildMode => Decompressed::Built(decompressed),
        CountMode => Decompressed::Counted(count),
    }
}

fn main() {
    let sequence = load_compressed_sequence("input.txt");
    let compressed_len = &sequence.len();
    let expanded_1 = decompress(&sequence, Mode::BuildMode, false);
    let len_1 = expanded_1.len();
    println!("Part 1: Expanded {} characters to {}.", compressed_len, len_1);
    let expanded_2 = decompress(&sequence, Mode::CountMode, true);
    let len_2 = expanded_2.len();
    println!("Part 2: Expanded {} characters to {}.", compressed_len, len_2);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_1() {
        use Decompressed::*;
        let compressed = "ADVENT";
        let expected = Built("ADVENT".to_string());
        assert_eq!(expected,decompress(compressed, Mode::BuildMode, false));
    }

    #[test]
    fn test_example_1_count() {
        use Decompressed::*;
        let compressed = "ADVENT";
        let expected = Counted("ADVENT".len());
        assert_eq!(expected, decompress(compressed, Mode::CountMode, false));
    }

    #[test]
    fn test_example_2() {
        use Decompressed::*;
        let compressed = "A(1x5)BC";
        let expected = Built("ABBBBBC".to_string());
        assert_eq!(expected, decompress(compressed, Mode::BuildMode, false));
    }

    #[test]
    fn test_example_2_count() {
        use Decompressed::*;
        let compressed = "A(1x5)BC";
        let expected = Counted("ABBBBBC".len());
        assert_eq!(expected, decompress(compressed, Mode::CountMode, false));
    }

    #[test]
    fn test_example_3() {
        use Decompressed::*;
        let compressed = "(3x3)XYZ";
        let expected = Built("XYZXYZXYZ".to_string());
        assert_eq!(expected, decompress(compressed, Mode::BuildMode, false));
    }

    #[test]
    fn test_example_3_count() {
        use Decompressed::*;
        let compressed = "(3x3)XYZ";
        let expected = Counted("XYZXYZXYZ".len());
        assert_eq!(expected, decompress(compressed, Mode::CountMode, false));
    }

    #[test]
    fn test_example_4() {
        use Decompressed::*;
        let compressed = "A(2x2)BCD(2x2)EFG";
        let expected = Built("ABCBCDEFEFG".to_string());
        assert_eq!(expected, decompress(compressed, Mode::BuildMode, false));
    }

    #[test]
    fn test_example_4_count() {
        use Decompressed::*;
        let compressed = "A(2x2)BCD(2x2)EFG";
        let expected = Counted("ABCBCDEFEFG".len());
        assert_eq!(expected, decompress(compressed, Mode::CountMode, false));
    }

    #[test]
    fn test_example_5() {
        use Decompressed::*;
        let compressed = "(6x1)(1x3)A";
        let expected = Built("(1x3)A".to_string());
        assert_eq!(expected, decompress(compressed, Mode::BuildMode, false));
    }

    #[test]
    fn test_example_5_count() {
        use Decompressed::*;
        let compressed = "(6x1)(1x3)A";
        let expected = Counted("(1x3)A".len());
        assert_eq!(expected, decompress(compressed, Mode::CountMode, false));
    }

    #[test]
    fn test_example_6() {
        use Decompressed::*;
        let compressed = "X(8x2)(3x3)ABCY";
        let expected = Built("X(3x3)ABC(3x3)ABCY".to_string());
        assert_eq!(expected, decompress(compressed, Mode::BuildMode, false));
    }

    #[test]
    fn test_example_6_count() {
        use Decompressed::*;
        let compressed = "X(8x2)(3x3)ABCY";
        let expected = Counted("X(3x3)ABC(3x3)ABCY".len());
        assert_eq!(expected, decompress(compressed, Mode::CountMode, false));
    }

    #[test]
    fn test_whitespace_is_skipped() {
        use Decompressed::*;
        let compressed = "X(8x2)(3x3)ABCY Z";
        let expected = Built("X(3x3)ABC(3x3)ABCYZ".to_string());
        assert_eq!(expected, decompress(compressed, Mode::BuildMode, false));
    }

    #[test]
    fn test_whitespace_is_skipped_count() {
        use Decompressed::*;
        let compressed = "X(8x2)(3x3)ABCY Z";
        let expected = Counted("X(3x3)ABC(3x3)ABCYZ".len());
        assert_eq!(expected, decompress(compressed, Mode::CountMode, false));
    }

    #[test]
    fn test_whitespace_is_skipped_in_marker_sequence() {
        use Decompressed::*;
        let compressed = "X(8x2)(3x3)A BCY";
        let expected = Built("X(3x3)AB(3x3)ABCY".to_string());
        assert_eq!(expected, decompress(compressed, Mode::BuildMode, false));
    }

    #[test]
    fn test_whitespace_is_skipped_in_marker_sequence_count() {
        use Decompressed::*;
        let compressed = "X(8x2)(3x3)A BCY";
        let expected = Counted("X(3x3)AB(3x3)ABCY".len());
        assert_eq!(expected, decompress(compressed, Mode::CountMode, false));
    }

    #[test]
    fn test_part1_answer() {
        let sequence = load_compressed_sequence("input.txt");
        let expanded = decompress(&sequence, Mode::BuildMode, false);
        assert_eq!(expanded.len(), 120765);
    }

    #[test]
    fn test_part1_answer_count() {
        let sequence = load_compressed_sequence("input.txt");
        let expanded = decompress(&sequence, Mode::CountMode, false);
        assert_eq!(expanded.len(), 120765);
    }

}
