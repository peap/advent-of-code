use std::fs::File;
use std::io::{BufRead, BufReader};

struct Triangle {
    a: i32,
    b: i32,
    c: i32,
}

impl Triangle {
    fn from_line(line: String) -> Triangle {
        let a = line[..5].trim().parse::<i32>().unwrap();
        let b = line[5..10].trim().parse::<i32>().unwrap();
        let c = line[10..].trim().parse::<i32>().unwrap();
        Triangle { a: a, b: b, c: c }
    }

    fn is_valid(&self) -> bool {
        self.a + self.b > self.c &&
        self.b + self.c > self.a &&
        self.c + self.a > self.b
    }
}

fn load_triangles(filename: &'static str) -> Vec<Triangle> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut triangles = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(text) => triangles.push(Triangle::from_line(text)),
            _ => ()
        }
    }
    triangles
}

fn count_valid(triangles: &Vec<Triangle>) -> i32 {
    triangles.iter().fold(0, |count, t| if t.is_valid() { count + 1 } else { count })
}

fn main() {
    let triangles = load_triangles("input.txt");
    let num_valid = count_valid(&triangles);
    println!("Loaded {} triangles; {} are valid.", triangles.len(), num_valid);
}
