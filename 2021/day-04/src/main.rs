use common::InputReader;

use ansi_term::Style;

#[derive(Clone)]
struct Space {
    num: i64,
    marked: bool,
}

impl Space {
    fn from_str(num: &str) -> Space {
        Space {
            num: num.parse().unwrap(),
            marked: false,
        }
    }

    fn mark(&mut self) {
        self.marked = true;
    }
}

#[derive(Clone)]
struct Board {
    rows: Vec<Vec<Space>>,
    winning_num: Option<i64>,
    won: bool,
}

impl Board {
    fn new() -> Board {
        Board {
            rows: vec![],
            winning_num: None,
            won: false,
        }
    }

    fn add_row(&mut self, row: String) {
        self.rows.push(
            row.split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| Space::from_str(n))
                .collect(),
        );
    }

    fn mark_spaces(&mut self, num: &i64) {
        for row in self.rows.iter_mut() {
            for space in row.iter_mut() {
                if space.num == *num {
                    space.mark();
                }
            }
        }
    }

    fn just_won(&self) -> bool {
        let mut cols_marked = vec![true; self.rows[0].len()];
        for row in self.rows.iter() {
            let mut row_marked = true;
            for (i, space) in row.iter().enumerate() {
                row_marked = row_marked && space.marked;
                cols_marked[i] = cols_marked[i] && space.marked;
            }
            if row_marked {
                return true;
            }
        }
        cols_marked.iter().any(|c| *c)
    }

    fn score(&self) -> i64 {
        let mut score = 0;
        for row in self.rows.iter() {
            for space in row.iter() {
                if !space.marked {
                    score += space.num;
                }
            }
        }
        score
    }

    fn final_score(&self) -> i64 {
        self.score() * self.winning_num.unwrap()
    }

    fn print(&self) {
        for row in self.rows.iter() {
            for space in row.iter() {
                if space.marked {
                    print!(
                        "{} ",
                        Style::new().bold().paint(format!("{:>2}", space.num))
                    );
                } else {
                    print!("{:>2} ", space.num);
                }
            }
            println!();
        }
    }
}

fn parse_bingo(bingo: &[String]) -> (Vec<i64>, Vec<Board>) {
    let nums: Vec<i64> = bingo[0].split(',').map(|s| s.parse().unwrap()).collect();
    let mut boards = vec![];
    let mut board = Board::new();
    for line in bingo[2..].iter() {
        if line.is_empty() {
            boards.push(board);
            board = Board::new();
            continue;
        } else {
            board.add_row(String::from(line));
        }
    }
    boards.push(board);
    (nums, boards)
}

fn play_bingo(nums: Vec<i64>, mut boards: Vec<Board>) -> Vec<Board> {
    let mut winners = vec![];
    for num in nums.iter() {
        for board in boards.iter_mut() {
            board.mark_spaces(num);
            if !board.won && board.just_won() {
                board.winning_num = Some(*num);
                board.won = true;
                winners.push(board.clone());
            }
        }
    }
    winners
}

fn main() {
    let bingo = InputReader::new("input.txt").string_lines();
    let (nums, boards) = parse_bingo(&bingo);
    let winners = play_bingo(nums, boards);
    let (first, last) = (winners.first().unwrap(), winners.last().unwrap());
    first.print();
    println!("Part 1: final score {}", first.final_score());
    last.print();
    println!("Part 2: final score {}", last.final_score());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let bingo: Vec<String> = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ]
        .iter()
        .map(|&s| s.into())
        .collect();
        let (nums, boards) = parse_bingo(&bingo);
        let winners = play_bingo(nums, boards);
        let (first, last) = (winners.first().unwrap(), winners.last().unwrap());
        assert_eq!(first.winning_num, Some(24));
        assert_eq!(first.score(), 188);
        assert_eq!(last.winning_num, Some(13));
        assert_eq!(last.score(), 148);
    }

    #[test]
    fn test_part1() {
        let bingo = InputReader::new("input.txt").string_lines();
        let (nums, boards) = parse_bingo(&bingo);
        let winners = play_bingo(nums, boards);
        let (first, last) = (winners.first().unwrap(), winners.last().unwrap());
        assert_eq!(first.winning_num, Some(5));
        assert_eq!(first.score(), 1137);
        assert_eq!(last.winning_num, Some(49));
        assert_eq!(last.score(), 430);
    }
}
