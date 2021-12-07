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

    pub fn u8_line(&self) -> Vec<u8> {
        self.string_line()
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
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
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
