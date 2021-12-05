use std::collections::{HashSet, VecDeque};

const SALT: u64 = 1358;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new(x: u8, y: u8) -> Point {
        Point { x: x, y: y }
    }

    fn get_valid_neighbors(&self) -> Vec<Point> {
        let mut neighbors = Vec::new();
        if let Some(x) = self.x.checked_sub(1) {
            neighbors.push(Point::new(x, self.y));
        }
        if let Some(x) = self.x.checked_add(1) {
            neighbors.push(Point::new(x, self.y));
        }
        if let Some(y) = self.y.checked_sub(1) {
            neighbors.push(Point::new(self.x, y));
        }
        if let Some(y) = self.y.checked_add(1) {
            neighbors.push(Point::new(self.x, y));
        }
        neighbors.into_iter().filter(|p| !p.is_wall()).collect()
    }

    fn is_wall(&self) -> bool {
        let x = self.x as u64;
        let y = self.y as u64;
        let value: u64 = x * x + 3 * x + 2 * x * y + y + y * y + SALT;
        value.count_ones() % 2 == 1
    }
}

pub fn find_shortest_path(start: &Point, end: &Point) -> u32 {
    let mut q: VecDeque<(Point, u32)> = VecDeque::new();
    let mut visited: HashSet<Point> = HashSet::new();
    for point in start.get_valid_neighbors() {
        q.push_back((point, 1));
    }
    visited.insert(start.clone());
    let mut part2_printed = false;
    while !q.is_empty() {
        let (point, num) = q.pop_front().unwrap();
        if !part2_printed && num >= 50 {
            println!("Have seen {} points with under 50 moves.", visited.len());
            part2_printed = true;
        }
        if point == *end {
            return num;
        } else {
            for to_visit in point.get_valid_neighbors() {
                if !visited.contains(&to_visit) {
                    q.push_back((to_visit.clone(), num + 1));
                    visited.insert(to_visit);
                }
            }
        }
    }
    u32::max_value()
}

fn main() {
    let start = Point::new(1, 1);
    let end = Point::new(31, 39);
    let num_steps = find_shortest_path(&start, &end);
    println!(
        "Stating at {:?}, can walk to {:?} in {} steps.",
        &start, &end, num_steps
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_answer() {
        let start = Point::new(1, 1);
        let end = Point::new(31, 39);
        assert_eq!(96, find_shortest_path(&start, &end));
    }
}
