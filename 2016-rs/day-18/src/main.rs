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

pub fn count_safe_tiles<'a>(start: &'a str, n_rows: usize) -> usize {
    let mut rows = vec![start.to_string()];
    for i in 1..n_rows {
        let row = {
            // introduce scope to allow immutable borrow of rows
            generate_row_from(&rows[i - 1])
        };
        rows.push(row);
    }
    rows.iter().fold(0, |acc, r| acc + r.chars().filter(|c| *c == SAFE).count())
}

fn main() {
    let count = count_safe_tiles(ROW1, 40);
    println!("Part 1: there are {} safe tiles", count);
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
}
