use common::{default_puzzle, Answer, InputReader, Puzzle};

fn part1(reader: &InputReader) -> Answer {
    let _lines: Vec<String> = reader.parsed_lines();
    0
}

fn part2(reader: &InputReader) -> Answer {
    let _lines: Vec<String> = reader.parsed_lines();
    0
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Transparent Origami");
    puzzle.set_part1(part1, "todo");
    puzzle.set_part2(part2, "todo");
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
