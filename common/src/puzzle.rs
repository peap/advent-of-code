use crate::InputReader;

#[macro_export]
macro_rules! default_puzzle {
    ( $desc:literal ) => {
        // Modules using this macro must be named `year-NNNN-day-NN`.
        {
            let _splits: Vec<&str> = env!("CARGO_PKG_NAME").split('-').collect();
            let _year = _splits[1].parse().unwrap();
            let _day = _splits[3].parse().unwrap();
            Puzzle::new(_year, _day, $desc, "input.txt")
        }
    };
}

pub type Answer = u64;
type Solver = fn(&InputReader) -> Answer;

pub struct Puzzle {
    title: String,
    reader: InputReader,
    part1: Option<Solver>,
    part2: Option<Solver>,
    part1_summary: Option<String>,
    part2_summary: Option<String>,
}

impl Puzzle {
    pub fn new(year: u16, day: u8, title: &str, input_path: &'static str) -> Self {
        Puzzle {
            title: format!("{}, Day {}: {}", year, day, title),
            reader: InputReader::new(input_path),
            part1: None,
            part2: None,
            part1_summary: None,
            part2_summary: None,
        }
    }

    pub fn get_reader(&self) -> &InputReader {
        &self.reader
    }

    pub fn set_part1(&mut self, summary: &str, func: Solver) {
        self.part1_summary = Some(summary.to_string());
        self.part1 = Some(func);
    }

    pub fn set_part2(&mut self, summary: &str, func: Solver) {
        self.part2_summary = Some(summary.to_string());
        self.part2 = Some(func);
    }

    pub fn run(&self) {
        println!("{}", self.title);
        let width = self.title.len();
        println!("{}", String::from_utf8(vec![b'='; width]).unwrap());
        if let Some(part1_fn) = self.part1 {
            let summary = self.part1_summary.clone().unwrap();
            let answer = part1_fn(self.get_reader());
            println!("Part 1 - {}: {}", summary, answer);
        }
        if let Some(part2_fn) = self.part2 {
            let summary = self.part2_summary.clone().unwrap();
            let answer = part2_fn(self.get_reader());
            println!("Part 2 - {}: {}", summary, answer);
        }
    }

    pub fn test_part1(&self, want: Answer) {
        if let Some(part1_fn) = self.part1 {
            assert_eq!(part1_fn(self.get_reader()), want);
        }
    }

    pub fn test_part2(&self, want: Answer) {
        if let Some(part2_fn) = self.part2 {
            assert_eq!(part2_fn(self.get_reader()), want);
        }
    }
}
