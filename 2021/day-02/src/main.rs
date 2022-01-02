use common::{default_puzzle, Puzzle};

fn process_commands(commands: Vec<String>, with_aim: bool) -> (u64, u64) {
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in commands.iter() {
        let splits: Vec<&str> = command.split(' ').collect();
        let dir = splits[0];
        let num: u64 = splits[1].parse().unwrap();
        match dir {
            "up" => {
                aim -= num;
                if !with_aim {
                    depth -= num;
                }
            }
            "down" => {
                aim += num;
                if !with_aim {
                    depth += num;
                }
            }
            "forward" => {
                horiz += num;
                if with_aim {
                    depth += aim * num;
                }
            }
            _ => {
                panic!("bad command");
            }
        }
    }
    (horiz, depth)
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Dive!");
    puzzle.set_part1("final position (h*d)", |reader| {
        let commands = reader.parsed_lines();
        let (horiz, depth) = process_commands(commands, false);
        horiz * depth
    });
    puzzle.set_part2("final position (h*d) (w/aim)", |reader| {
        let commands = reader.parsed_lines();
        let (horiz, depth) = process_commands(commands, true);
        horiz * depth
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
    fn test_process_commands() {
        let ex1: Vec<String> = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .iter()
        .map(|&s| s.into())
        .collect();
        assert_eq!(process_commands(ex1.clone(), false), (15, 10));
        assert_eq!(process_commands(ex1, true), (15, 60));
    }

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(2102357);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(2101031224);
    }
}
