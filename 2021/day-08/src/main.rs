use std::collections::{HashMap, HashSet};

use common::{default_puzzle, Puzzle};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Segment {
    Top,
    TopL,
    TopR,
    Middle,
    BottomL,
    BottomR,
    Bottom,
}

struct SegmentResolver {
    possible_wires: HashMap<Segment, HashSet<char>>,
    resolved_wires: HashMap<Segment, char>,
    codes: HashMap<String, String>,
}

impl SegmentResolver {
    fn new() -> Self {
        // let wires = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);
        let wires = HashSet::new();
        let possible_wires = HashMap::from([
            (Segment::Top, wires.clone()),
            (Segment::TopL, wires.clone()),
            (Segment::TopR, wires.clone()),
            (Segment::Middle, wires.clone()),
            (Segment::BottomL, wires.clone()),
            (Segment::BottomR, wires.clone()),
            (Segment::Bottom, wires),
        ]);
        let resolved_wires = HashMap::new();
        let codes = HashMap::new();
        SegmentResolver {
            possible_wires,
            resolved_wires,
            codes,
        }
    }

    fn set_code(&mut self, code: &str, digit: &str) {
        self.codes.insert(code.to_string(), digit.to_string());
    }

    fn add_codes(&mut self, codes: Vec<String>) {
        use Segment::*;

        // Code lengths to digits:
        //   * 2 -> 1
        //   * 3 -> 7
        //   * 4 -> 4
        //   * 5 -> 2, 3, 5
        //   * 6 -> 0, 6, 9
        //   * 7 -> 8
        let one = codes.iter().find(|&c| c.len() == 2).unwrap();
        self.set_code(one, "1");
        let four = codes.iter().find(|&c| c.len() == 4).unwrap();
        self.set_code(four, "4");
        let seven = codes.iter().find(|&c| c.len() == 3).unwrap();
        self.set_code(seven, "7");
        let eight = codes.iter().find(|&c| c.len() == 7).unwrap();
        self.set_code(eight, "8");

        // 7 diff 1 -> Top
        self.resolve_wire(vec![Top], seven, one);

        // (4 diff 1 -> TopL, Middle)
        //    int
        // (2 int 3 int 5 -> ~Top~, Middle, Bottom)
        //    -> Middle -> TopL -> Bottom
        self.resolve_wire(vec![TopL, Middle], four, one);
        let maybe235: Vec<String> = codes.iter().filter(|&c| c.len() == 5).cloned().collect();
        let mut intersect235 = intersection(maybe235.clone());
        intersect235.remove(&self.resolved_wires[&Top]);
        let middle = self.possible_wires[&Middle]
            .intersection(&intersect235)
            .into_iter()
            .next()
            .unwrap();
        self.resolved_wires.insert(Middle, *middle);
        let topl = self.possible_wires[&Middle]
            .iter()
            .find(|&w| w != middle)
            .unwrap();
        self.resolved_wires.insert(TopL, *topl);
        let bottom = intersect235.iter().find(|&w| w != middle).unwrap();
        self.resolved_wires.insert(Bottom, *bottom);

        // 5 diff (4 known wires) -> BottomR
        let mut known: HashSet<char> = HashSet::from([
            self.resolved_wires[&Top],
            self.resolved_wires[&Middle],
            self.resolved_wires[&Bottom],
            self.resolved_wires[&TopL],
        ]);
        let five = maybe235
            .iter()
            .find(|&c| {
                let candidate: HashSet<char> = c.chars().collect();
                candidate.difference(&known).count() == 1
            })
            .unwrap();
        self.set_code(five, "5");
        self.resolve_wire(vec![BottomR], five, &known.iter().collect::<String>());
        known.insert(self.resolved_wires[&BottomR]);

        // 1 diff BottomR -> TopR
        self.resolve_wire(vec![TopR], one, &self.resolved_wires[&BottomR].to_string());
        known.insert(self.resolved_wires[&TopR]);
        // Remaining wire -> BottomL
        let bottoml = "abcdefg".chars().find(|&c| !known.contains(&c)).unwrap();
        self.resolved_wires.insert(BottomL, bottoml);

        // Figure remaining codes from segments.
        self.resolve_from_segments("0");
        self.resolve_from_segments("2");
        self.resolve_from_segments("3");
        self.resolve_from_segments("6");
        self.resolve_from_segments("9");
    }

    fn resolve_from_segments(&mut self, digit: &str) {
        use Segment::*;
        let segments: Vec<Segment> = match digit {
            "0" => vec![Top, TopL, TopR, BottomL, BottomR, Bottom],
            "1" => vec![TopR, BottomR],
            "2" => vec![Top, TopR, Middle, BottomL, Bottom],
            "3" => vec![Top, TopR, Middle, BottomR, Bottom],
            "4" => vec![TopL, TopR, Middle, BottomR],
            "5" => vec![Top, TopL, Middle, BottomR, Bottom],
            "6" => vec![Top, TopL, Middle, BottomL, BottomR, Bottom],
            "7" => vec![Top, TopR, BottomR],
            "8" => vec![Top, TopL, TopR, Middle, BottomL, BottomR, Bottom],
            "9" => vec![Top, TopL, TopR, Middle, BottomR, Bottom],
            _ => panic!("Bad digit {}", digit),
        };
        let mut wires: Vec<char> = segments
            .iter()
            .map(|s| *self.resolved_wires.get(s).unwrap())
            .collect();
        wires.sort_unstable();
        self.set_code(&String::from_iter(wires), digit);
    }

    fn resolve_wire(&mut self, segs: Vec<Segment>, code1: &str, code2: &str) {
        let longer: HashSet<char> = code1.chars().collect();
        let shorter: HashSet<char> = code2.chars().collect();
        let diff: Vec<&char> = longer.difference(&shorter).collect();
        if diff.len() == 1 {
            self.resolved_wires
                .insert(segs[0], *diff.into_iter().next().unwrap());
        } else {
            for seg in segs.iter() {
                if self.resolved_wires.get(seg).is_none() {
                    if let Some(wires) = self.possible_wires.get_mut(seg) {
                        for wire in diff.iter() {
                            wires.insert(**wire);
                        }
                    }
                }
            }
        }
    }

    fn parse(&self, digit_codes: Vec<String>) -> u64 {
        let digits: String = digit_codes
            .iter()
            .map(|c| self.codes.get(c).unwrap().to_string())
            .collect::<Vec<String>>()
            .join("");
        digits.parse().unwrap()
    }
}

fn intersection(codes: Vec<String>) -> HashSet<char> {
    let mut int: HashSet<char> = codes[0].chars().collect();
    for code in codes.iter().skip(1) {
        let code_set: HashSet<char> = code.chars().collect();
        int = int.intersection(&code_set).copied().collect();
    }
    int
}

fn sorted_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

fn parse_line(line: &str) -> (Vec<String>, Vec<String>) {
    let split: Vec<&str> = line.split('|').collect();
    let codes = split[0].trim().split(' ').map(sorted_string).collect();
    let digits = split[1].trim().split(' ').map(sorted_string).collect();
    (codes, digits)
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Seven Segment Search");
    puzzle.set_part1("number of 1|4|7|8", |reader| {
        let lines: Vec<String> = reader.parsed_lines();
        let mut count = 0;
        for line in lines.iter() {
            let (_, out_values) = parse_line(line);
            count += out_values
                .iter()
                .filter(|v| matches!(v.len(), 2 | 3 | 4 | 7))
                .count();
        }
        count as u64
    });
    puzzle.set_part2("sum of outputs", |reader| {
        let lines: Vec<String> = reader.parsed_lines();
        let mut sum = 0;
        for line in lines.iter() {
            let (codes, digits) = parse_line(line);
            let mut resolver = SegmentResolver::new();
            resolver.add_codes(codes);
            sum += resolver.parse(digits);
        }
        sum
    });
    puzzle
}

fn main() {
    get_puzzle().run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(554);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(990964);
    }
}
