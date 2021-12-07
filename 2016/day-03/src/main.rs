use std::str::FromStr;

use common::{BadInput, InputReader};

struct Triangle {
    a: i32,
    b: i32,
    c: i32,
}

impl FromStr for Triangle {
    type Err = BadInput;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (a, b, c) = parse_line(line);
        Ok(Triangle { a, b, c })
    }
}

impl Triangle {
    fn from_array(sides: [i32; 3]) -> Triangle {
        Triangle {
            a: sides[0],
            b: sides[1],
            c: sides[2],
        }
    }

    fn is_valid(&self) -> bool {
        self.a + self.b > self.c && self.b + self.c > self.a && self.c + self.a > self.b
    }
}

fn parse_line(line: &str) -> (i32, i32, i32) {
    let a = line[..5].trim().parse::<i32>().unwrap();
    let b = line[5..10].trim().parse::<i32>().unwrap();
    let c = line[10..].trim().parse::<i32>().unwrap();
    (a, b, c)
}

fn load_triangles_2(lines: Vec<String>) -> Vec<Triangle> {
    let mut triangles = Vec::new();
    let mut idx = 0;
    let mut t1 = [0; 3];
    let mut t2 = [0; 3];
    let mut t3 = [0; 3];
    for line in lines.iter() {
        let (a, b, c) = parse_line(line);
        t1[idx] = a;
        t2[idx] = b;
        t3[idx] = c;
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

fn count_valid(triangles: &[Triangle]) -> i32 {
    triangles
        .iter()
        .fold(0, |count, t| if t.is_valid() { count + 1 } else { count })
}

fn main() {
    let triangles1 = InputReader::new("input.txt").parsed_lines();
    let num1 = count_valid(&triangles1);
    println!(
        "Part 1: {} triangles; {} are valid.",
        &triangles1.len(),
        num1
    );
    let lines = InputReader::new("input.txt").parsed_lines();
    let triangles2 = load_triangles_2(lines);
    let num2 = count_valid(&triangles2);
    println!(
        "Part 2: {} triangles; {} are valid.",
        &triangles2.len(),
        num2
    );
}
