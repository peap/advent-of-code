pub const ROW1: &'static str =
    ".^.^..^......^^^^^...^^^...^...^....^^.^...^.^^^^....^...^^.^^^...^^^^\
     .^^.^.^^..^.^^^..^^^^^^.^^^..^";

const SAFE: char = '.';
const TRAP: char = '^';

fn generate_row_from<'a>(previous: &'a str) -> String {
    let mut new_row = String::new();
    let prev: Vec<char> = previous.chars().collect();
    for i in 0..prev.len() {
        let left = if i > 0 { prev[i - 1] } else { SAFE };
        let center = prev[i];
        let right = if i < prev.len() - 1 { prev[i + 1] } else { SAFE };
        let pattern = format!("{}{}{}", left, center, right);
        let new_char = match pattern.as_ref() {
            "^^." => TRAP,
            ".^^" => TRAP,
            "^.." => TRAP,
            "..^" => TRAP,
            _ => SAFE,
        };
        new_row.push(new_char);
    }
    new_row
}

fn count_safe<'a>(row: &'a str) -> usize {
    row.chars().filter(|c| *c == SAFE).count()
}

pub fn count_safe_tiles<'a>(start: &'a str, n_rows: usize) -> usize {
    let mut count = count_safe(start);
    let mut previous_row = start.to_string();
    for _ in 1..n_rows {
        let row = generate_row_from(&previous_row);
        count += count_safe(&row);
        previous_row = row;
    }
    count
}

fn main() {
    let count1 = count_safe_tiles(ROW1, 40);
    println!("Part 1: there are {} safe tiles", count1);
    let count2 = count_safe_tiles(ROW1, 400_000);
    println!("Part 2: there are {} safe tiles", count2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let count = count_safe_tiles(".^^.^.^^^^", 10);
        assert_eq!(count, 38);
    }

    #[test]
    fn test_part_1() {
        let count = count_safe_tiles(ROW1, 40);
        assert_eq!(count, 1987);
    }

    #[test]
    fn test_part_2() {
        let count = count_safe_tiles(ROW1, 400_000);
        assert_eq!(count, 19984714);
    }
}
