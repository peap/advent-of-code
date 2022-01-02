use common::{default_puzzle, Puzzle};

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("<NAME>");
    puzzle.set_part1("todo", |reader| {
        let _lines: Vec<String> = reader.parsed_lines();
        0
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
