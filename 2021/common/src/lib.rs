use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct InputReader {
    path: &'static str,
}

impl InputReader {
    pub fn new(path: &'static str) -> InputReader {
        InputReader{path}
    }

    pub fn string_lines(&self) -> Vec<String> {
        let file = File::open(&self.path).unwrap();
        let reader = BufReader::new(file);
        reader.lines().map(|l| l.unwrap()).collect()
    }

    pub fn i64_lines(&self) -> Vec<i64> {
        self.string_lines().iter().map(|l| l.parse().unwrap()).collect()
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
