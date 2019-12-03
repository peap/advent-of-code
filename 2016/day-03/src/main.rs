use std::fs::File;
use std::io::{BufRead, BufReader};

struct Triangle {
    a: i32,
    b: i32,
    c: i32,
}

impl Triangle {
    fn from_array(sides: [i32; 3]) -> Triangle {
        Triangle { a: sides[0], b: sides[1], c: sides[2] }
    }

    fn from_line(line: String) -> Triangle {
        let (a, b, c) = parse_line(line);
        Triangle { a: a, b: b, c: c }
    }

    fn is_valid(&self) -> bool {
        self.a + self.b > self.c &&
        self.b + self.c > self.a &&
        self.c + self.a > self.b
    }
}

fn parse_line(line: String) -> (i32, i32, i32) {
    let a = line[..5].trim().parse::<i32>().unwrap();
    let b = line[5..10].trim().parse::<i32>().unwrap();
    let c = line[10..].trim().parse::<i32>().unwrap();
    (a, b, c)
}

fn load_triangles_1(filename: &'static str) -> Vec<Triangle> {
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

fn load_triangles_2(filename: &'static str) -> Vec<Triangle> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut triangles = Vec::new();
    let mut idx = 0;
    let mut t1 = [0; 3];
    let mut t2 = [0; 3];
    let mut t3 = [0; 3];
    for line in reader.lines() {
        match line {
            Ok(text) => {
                let (a, b, c) = parse_line(text);
                t1[idx] = a;
                t2[idx] = b;
                t3[idx] = c;
            }
            _ => ()
        }
        idx += 1;
        if idx == 3 {
            triangles.push(Triangle::from_array(t1));
            triangles.push(Triangle::from_array(t2));
            triangles.push(Triangle::from_array(t3));
            idx = 0;
        }
    }
    triangles
}

fn count_valid(triangles: &Vec<Triangle>) -> i32 {
    triangles.iter().fold(0, |count, t| if t.is_valid() { count + 1 } else { count })
}

fn main() {
    let triangles1 = load_triangles_1("input.txt");
    let num1 = count_valid(&triangles1);
    println!("Part 1: {} triangles; {} are valid.", &triangles1.len(), num1);
    let triangles2 = load_triangles_2("input.txt");
    let num2 = count_valid(&triangles2);
    println!("Part 2: {} triangles; {} are valid.", &triangles2.len(), num2);
}
