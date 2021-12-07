use std::str::FromStr;

use common::{BadInput, InputReader};

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
        Point { x, y }
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
        Span { a, b }
    }

    fn contains_x(&self, x: i32) -> bool {
        if self.a.x < self.b.x {
            self.a.x <= x && x <= self.b.x
        } else {
            self.b.x <= x && x <= self.a.x
        }
    }

    fn contains_y(&self, y: i32) -> bool {
        if self.a.y < self.b.y {
            self.a.y <= y && y <= self.b.y
        } else {
            self.b.y <= y && y <= self.a.y
        }
    }

    fn get_intersection(&self, other: &Span) -> Option<Point> {
        if self.is_horizontal() && other.is_horizontal() {
            return None;
        }
        if self.is_vertical() && other.is_vertical() {
            return None;
        }
        if self.is_horizontal() {
            if self.contains_x(other.a.x) && other.contains_y(self.a.y) {
                return Some(Point::new(other.a.x, self.a.y));
            }
        } else if self.contains_y(other.a.y) && other.contains_x(self.a.x) {
            return Some(Point::new(self.a.x, other.a.y));
        }
        None
    }

    fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    fn len(&self) -> i32 {
        if self.is_horizontal() {
            (self.b.x - self.a.x).abs()
        } else {
            (self.b.y - self.a.y).abs()
        }
    }
}

struct Wire {
    paths: Vec<Path>,
}

impl FromStr for Wire {
    type Err = BadInput;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(Wire::new(string))
    }
}

impl Wire {
    fn new(s: &str) -> Wire {
        let paths = s.split(',').map(|p| Path::new(p)).collect();
        Wire { paths }
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

#[derive(Debug, Eq, PartialEq)]
struct Intersection {
    point: Point,
    steps: i32,
}

impl Intersection {
    fn new(point: Point, steps: i32) -> Intersection {
        Intersection { point, steps }
    }

    fn taxicab(&self) -> i32 {
        self.point.taxicab()
    }
}

fn find_intersections(wire1: &Wire, wire2: &Wire) -> Vec<Intersection> {
    let mut intersections = vec![];
    let spans1 = wire1.spans();
    let spans2 = wire2.spans();
    let mut steps1 = 0;
    for s1 in spans1.iter() {
        let mut steps2 = 0;
        for s2 in spans2.iter() {
            if let Some(p) = s1.get_intersection(s2) {
                if p.x == 0 && p.y == 0 {
                    continue;
                }
                let mut steps = steps1 + steps2;
                steps += Span::new(s1.a.clone(), p.clone()).len();
                steps += Span::new(s2.a.clone(), p.clone()).len();
                intersections.push(Intersection::new(p, steps));
            }
            steps2 += s2.len();
        }
        steps1 += s1.len();
    }
    intersections
}

fn setup() -> Vec<Intersection> {
    let wires = InputReader::new("input.txt").parsed_lines();
    find_intersections(&wires[0], &wires[1])
}

fn part1() -> i32 {
    let intersections = setup();
    intersections.iter().map(|i| i.taxicab()).min().unwrap()
}

fn part2() -> i32 {
    let intersections = setup();
    intersections.iter().map(|i| i.steps).min().unwrap()
}

fn main() {
    println!(
        "Part 1: The closest intersection is {} away (taxicab)",
        part1()
    );
    println!(
        "Part 2: The closest intersection is {} away (steps)",
        part2()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_intersections_ex1() {
        let wire1 = Wire::new("R8,U5,L5,D3");
        let wire2 = Wire::new("U7,R6,D4,L4");
        let intersections = find_intersections(&wire1, &wire2);
        assert_eq!(
            intersections,
            vec![
                Intersection::new(Point::new(6, 5), 30),
                Intersection::new(Point::new(3, 3), 40),
            ]
        );
    }

    #[test]
    fn test_find_intersections_ex2() {
        let wire1 = Wire::new("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire2 = Wire::new("U62,R66,U55,R34,D71,R55,D58,R83");
        let intersections = find_intersections(&wire1, &wire2);
        let min_taxicab = intersections.iter().map(|i| i.taxicab()).min().unwrap();
        let min_steps = intersections.iter().map(|i| i.steps).min().unwrap();
        assert_eq!(min_taxicab, 159);
        assert_eq!(min_steps, 610);
    }

    #[test]
    fn test_find_intersections_ex3() {
        let wire1 = Wire::new("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let wire2 = Wire::new("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let intersections = find_intersections(&wire1, &wire2);
        let min_taxicab = intersections.iter().map(|i| i.taxicab()).min().unwrap();
        let min_steps = intersections.iter().map(|i| i.steps).min().unwrap();
        assert_eq!(min_taxicab, 135);
        assert_eq!(min_steps, 410);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1674);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 14012);
    }
}
