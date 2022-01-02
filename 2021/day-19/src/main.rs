use common::{default_puzzle, Puzzle};

#[derive(Clone, Eq, Hash, PartialEq)]
struct Beacon {
    x: i32, // increasing rightward
    y: i32, // increasing upward
    z: i32, // increasing away from us
}

impl Beacon {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn distance_to(&self, other: &Self) -> (i32, i32, i32) {
        (other.x - self.x, other.y - self.y, other.z - self.z)
    }

    fn get_rotations(&self) -> Vec<Self> {
        let (x, y, z) = (self.x, self.y, self.z);
        vec![
            // y is up, x is right, z is back; rotate around y
            Self::new(x, y, z),
            Self::new(-z, y, x),
            Self::new(-x, y, -z),
            Self::new(z, y, -x),
            // y is down, x is left, z is back; rotate around y
            Self::new(-x, -y, z),
            Self::new(-z, -y, -x),
            Self::new(x, -y, -z),
            Self::new(z, -y, x),
            // x is up, y is left, z is back; rotate around x
            Self::new(-y, x, z),
            Self::new(-z, x, -y),
            Self::new(y, x, -z),
            Self::new(z, x, y),
            // x is up, y is right, z is back; rotate around x
            Self::new(y, -x, z),
            Self::new(-z, -x, y),
            Self::new(-y, -x, -z),
            Self::new(z, -x, -y),
            // z is up, x is right, y is forward; rotate around z
            Self::new(x, z, -y),
            Self::new(y, z, x),
            Self::new(-x, z, y),
            Self::new(-y, z, -x),
            // z is down, x is left, y is forward; rotate around z
            Self::new(-x, -z, -y),
            Self::new(y, -z, -x),
            Self::new(x, -z, y),
            Self::new(-y, -z, x),
        ]
    }
}

struct Scanner {
    id: u8,
    beacons: Vec<Beacon>,
}

impl Scanner {
    fn new(id: u8) -> Self {
        Self {
            id,
            beacons: vec![],
        }
    }

    fn add_beacon(&mut self, coords: &str) {
        let xyz: Vec<&str> = coords.split(',').collect();
        let x = xyz[0].parse().unwrap();
        let y = xyz[1].parse().unwrap();
        let z = xyz[2].parse().unwrap();
        self.beacons.push(Beacon::new(x, y, z));
    }

    fn find_overlap(&self, other: &Self) {}
}

struct BeaconMap {}

impl BeaconMap {
    fn new(scanners: Vec<Scanner>) -> Self {
        Self {}
    }

    fn count_beacons(&self) -> u64 {
        0
    }
}

fn scanners_from_lines(lines: Vec<String>) -> Vec<Scanner> {
    let mut scanners = vec![];
    let mut i = 0;
    for line in lines.iter() {
        if line.starts_with("--- scanner ") {
            let id_str = line
                .trim_start_matches("--- scanner ")
                .trim_end_matches(" ---");
            let id: u8 = id_str.parse().unwrap();
            scanners.push(Scanner::new(id));
        } else if line.is_empty() {
            i += 1;
        } else {
            scanners[i].add_beacon(line);
        }
    }
    scanners
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Beacon Scanner");
    puzzle.set_part1("number of beacons", |reader| {
        let lines: Vec<String> = reader.parsed_lines();
        let scanners = scanners_from_lines(lines);
        let beacon_map = BeaconMap::new(scanners);
        beacon_map.count_beacons()
    });
    puzzle.set_part2("todo", |reader| {
        let _lines: Vec<String> = reader.parsed_lines();
        0
    });
    puzzle
}

fn main() {
    get_puzzle().run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_beacon_get_rotations() {
        let beacon = Beacon::new(1, 2, 3);
        let rots = beacon.get_rotations();
        assert_eq!(rots.len(), 24);
        let set: HashSet<&Beacon> = HashSet::from_iter(rots.iter());
        assert_eq!(set.len(), 24);
    }

    #[test]
    fn test_example() {}

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(0);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(0);
    }
}
