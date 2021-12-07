use common::InputReader;

fn process_commands(commands: Vec<String>, with_aim: bool) -> (i64, i64) {
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in commands.iter() {
        let splits: Vec<&str> = command.split(' ').collect();
        let dir = splits[0];
        let num: i64 = splits[1].parse().unwrap();
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

fn main() {
    let commands = InputReader::new("input.txt").parsed_lines();
    let (horiz, depth) = process_commands(commands.clone(), false);
    let product = horiz * depth;
    println!("Part 1: h: {}, d: {}; h * d = {}", horiz, depth, product);
    let (horiz2, depth2) = process_commands(commands, true);
    let product2 = horiz2 * depth2;
    println!("Part 2: h: {}, d: {}; h * d = {}", horiz2, depth2, product2);
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
        let commands = InputReader::new("input.txt").parsed_lines();
        assert_eq!(process_commands(commands, false), (1927, 1091));
    }

    #[test]
    fn test_part2() {
        let commands = InputReader::new("input.txt").parsed_lines();
        assert_eq!(process_commands(commands, true), (1927, 1090312));
    }
}
