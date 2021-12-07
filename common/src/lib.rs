use std::fs::File;
use std::io::{BufRead, BufReader};

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

    pub fn string_lines(&self) -> Vec<String> {
        let file = File::open(&self.path).unwrap();
        let reader = BufReader::new(file);
        reader.lines().map(|l| l.unwrap()).collect()
    }

    pub fn i64_lines(&self) -> Vec<i64> {
        self.string_lines()
            .iter()
            .map(|l| l.parse().unwrap())
            .collect()
    }

    pub fn converted_lines<T: From<String>>(&self) -> Vec<T> {
        self.string_lines().into_iter().map(T::from).collect()
    }

    // =============================
    // Expanding a single line
    // =============================

    pub fn string_line(&self) -> String {
        self.string_lines()[0].to_string()
    }

    pub fn u8_line(&self) -> Vec<u8> {
        self.string_lines()[0]
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect()
    }

    pub fn csv_strings_line(&self) -> Vec<String> {
        self.string_lines()[0]
            .trim()
            .split(',')
            .map(|x| x.trim().to_string())
            .collect()
    }

    pub fn csv_u8_line(&self) -> Vec<u8> {
        self.string_lines()[0]
            .trim()
            .split(',')
            .map(|x| x.trim().parse::<u8>().unwrap())
            .collect()
    }

    pub fn csv_u64_line(&self) -> Vec<u64> {
        self.string_lines()[0]
            .trim()
            .split(',')
            .map(|x| x.trim().parse::<u64>().unwrap())
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
