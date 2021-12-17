use common::{default_puzzle, Answer, InputReader, Puzzle};

fn part1(reader: &InputReader) -> Answer {
    let _line: Vec<u8> = reader.digit_line(16);
    0
}

fn part2(reader: &InputReader) -> Answer {
    let _line: Vec<u8> = reader.digit_line(16);
    0
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Packet Decoder");
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
