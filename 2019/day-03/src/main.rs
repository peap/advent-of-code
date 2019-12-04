use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Direction {
        match s {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!("Unknown direction {}", s),
        }
    }
}

struct Path {
    direction: Direction,
    distance: i32,
}

impl Path {
    fn new(fragment: &str) -> Path {
        let (dir, dist) = fragment.split_at(1);
        Path {
            direction: Direction::from(dir),
            distance: dist.parse().unwrap(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn taxicab(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug)]
struct Span {
    a: Point,
    b: Point,
}

impl Span {
    fn new(a: Point, b: Point) -> Span {
        if a.x == b.x {
            if a.y < b.y {
                Span { a: a, b: b }
            } else {
                Span { a: b, b: a }
            }
        } else {
            if a.x < b.x {
                Span { a: a, b: b }
            } else {
                Span { a: b, b: a }
            }
        }
    }

    fn get_intersection(&self, other: &Span) -> Option<Point> {
        if self.is_horizontal() && other.is_horizontal() { return None; }
        if self.is_vertical() && other.is_vertical() { return None; }
        if self.is_horizontal() {
            let my_x_range = self.a.x..(self.b.x + 1);
            let their_y_range = other.a.y..(other.b.y + 1);
            if my_x_range.contains(&other.a.x) && their_y_range.contains(&self.a.y) {
                return Some(Point::new(other.a.x, self.a.y));
            }
        } else {
            let my_y_range = self.a.y..(self.b.y + 1);
            let their_x_range = other.a.x..(other.b.x + 1);
            if my_y_range.contains(&other.a.y) && their_x_range.contains(&self.a.x) {
                return Some(Point::new(self.a.x, other.a.y));
            }
        }
        None
    }

    fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }
}

struct Wire {
    paths: Vec<Path>,
}

impl Wire {
    fn new(s: &str) -> Wire {
        let paths = s.split(",").map(|p| Path::new(p)).collect();
        Wire { paths: paths }
    }

    fn spans(&self) -> Vec<Span> {
        let mut last = Point::new(0, 0);
        let mut spans = vec![];
        for path in self.paths.iter() {
            let mut next = Point::new(last.x, last.y);
            match path.direction {
                Direction::Up => next.y += path.distance,
                Direction::Right => next.x += path.distance,
                Direction::Down => next.y -= path.distance,
                Direction::Left => next.x -= path.distance,
            }
            spans.push(Span::new(last.clone(), next.clone()));
            last = next;
        }
        spans
    }
}

fn load_raw_strings(filename: &'static str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

fn find_intersections(wire1: &Wire, wire2: &Wire) -> Vec<Point> {
    let mut intersections = vec![];
    let spans1 = wire1.spans();
    let spans2 = wire2.spans();
    for s1 in spans1.iter() {
        for s2 in spans2.iter() {
            if let Some(p) = s1.get_intersection(s2) {
                if p.x != 0 && p.y != 0 {
                    intersections.push(p);
                }
            }
        }
    }
    intersections
}

fn part1() -> i32 {
    let raw_strings = load_raw_strings("input.txt");
    let wires: Vec<Wire> = raw_strings.iter().map(|s| Wire::new(s)).collect();
    let intersections = find_intersections(&wires[0], &wires[1]);
    intersections.iter().map(|i| i.taxicab()).min().unwrap()
}

fn main() {
    println!("Part 1: The closest intersection is {} away", part1())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_intersections_ex1() {
        let wire1 = Wire::new("R8,U5,L5,D3");
        let wire2 = Wire::new("U7,R6,D4,L4");
        let intersections = find_intersections(&wire1, &wire2);
        assert_eq!(intersections, vec![Point::new(6, 5), Point::new(3, 3)]);
    }

    #[test]
    fn test_find_intersections_ex2() {
        let wire1 = Wire::new("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire2 = Wire::new("U62,R66,U55,R34,D71,R55,D58,R83");
        let intersections = find_intersections(&wire1, &wire2);
        let closest = intersections.iter().map(|i| i.taxicab()).min().unwrap();
        assert_eq!(closest, 159);
    }

    #[test]
    fn test_find_intersections_ex3() {
        let wire1 = Wire::new("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let wire2 = Wire::new("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let intersections = find_intersections(&wire1, &wire2);
        let closest = intersections.iter().map(|i| i.taxicab()).min().unwrap();
        assert_eq!(closest, 135);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1674);
    }

}
