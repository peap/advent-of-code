use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct BadInput;

pub struct InputReader {
    path: &'static str,
}

impl InputReader {
    pub fn new(path: &'static str) -> InputReader {
        InputReader { path }
    }

    // =============================
    // One-to-one line parsing
    // =============================

    fn string_lines(&self) -> Vec<String> {
        let file = File::open(&self.path).unwrap();
        let reader = BufReader::new(file);
        reader.lines().map(|l| l.unwrap()).collect()
    }

    pub fn parsed_lines<T: FromStr>(&self) -> Vec<T>
    where
        <T as FromStr>::Err: Debug,
    {
        self.string_lines()
            .iter()
            .map(|l| l.parse().unwrap())
            .collect()
    }

    // =============================
    // Expanding a single line
    // =============================

    pub fn string_line(&self) -> String {
        self.string_lines().first().unwrap().to_string()
    }

    pub fn digit_line(&self, radix: u32) -> Vec<u8> {
        self.string_line()
            .trim()
            .chars()
            .map(|c| c.to_digit(radix).unwrap() as u8)
            .collect()
    }

    pub fn parsed_csv_line<T: FromStr>(&self) -> Vec<T>
    where
        <T as FromStr>::Err: Debug,
    {
        self.string_line()
            .split(',')
            .map(|x| x.trim().parse().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::InputReader;

    #[test]
    fn test_string_lines() {
        let lines: Vec<String> = InputReader::new("../2021/day-01/input.txt").parsed_lines();
        assert_eq!(lines.len(), 2000);
        assert_eq!(lines[0], "183".to_string());
        assert_eq!(lines[1], "185".to_string());
        assert_eq!(lines[1999], "9875".to_string());
    }

    #[test]
    fn test_u64_lines() {
        let lines: Vec<u64> = InputReader::new("../2021/day-01/input.txt").parsed_lines();
        assert_eq!(lines.len(), 2000);
        assert_eq!(lines[0], 183);
        assert_eq!(lines[1], 185);
        assert_eq!(lines[1999], 9875);
    }

    #[test]
    fn test_i64_lines() {
        let lines: Vec<i64> = InputReader::new("../2021/day-01/input.txt").parsed_lines();
        assert_eq!(lines.len(), 2000);
        assert_eq!(lines[0], 183);
        assert_eq!(lines[1], 185);
        assert_eq!(lines[1999], 9875);
    }

    #[test]
    fn test_string_line() {
        let line = InputReader::new("../2016/day-09/input.txt").string_line();
        assert_eq!(line.len(), 16919);
        assert_eq!(line.chars().next(), Some('('));
    }

    #[test]
    fn test_digit_line_base10() {
        let line = InputReader::new("../2019/day-08/input.txt").digit_line(10);
        assert_eq!(line.len(), 15000);
        assert_eq!(line[0..5], vec![2, 2, 2, 2, 0]);
    }

    #[test]
    fn test_digit_line_base16() {
        let line = InputReader::new("../2021/day-16/input.txt").digit_line(16);
        assert_eq!(line.len(), 1368);
        // A20D5
        assert_eq!(line[0..5], vec![10, 2, 0, 13, 5]);
    }

    #[test]
    fn test_parsed_csv_line() {
        let nums: Vec<u16> = InputReader::new("../2021/day-07/input.txt").parsed_csv_line();
        assert_eq!(nums.len(), 1000);
        assert_eq!(nums[0..5], vec![1101, 1, 29, 67, 1102]);
    }
}
