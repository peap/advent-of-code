// My input: "To continue, please consult the code grid in the manual. Enter
// the code at row 3010, column 3019."

pub const ROW: usize = 3010;
pub const COL: usize = 3019;

pub fn get_position_in_sequence(row: usize, col: usize) -> usize {
    assert!(row >= 1);
    assert!(col >= 1);
    let mut n = 1;
    let mut inc = 1;
    for _ in 0..(row - 1) {
        n += inc;
        inc += 1;
    }
    inc = row + 1;
    for _ in 0..(col - 1) {
        n += inc;
        inc += 1;
    }
    n
}

pub fn get_code(row: usize, col: usize) -> u64 {
    let nth = get_position_in_sequence(row, col);
    let mut code = 20151125;
    for _ in 1..nth {
        code = (code * 252533) % 33554393;
    }
    code
}

fn main() {
    let code = get_code(ROW, COL);
    println!("Part 1: The code at ({}, {}) is {}.", ROW, COL, code);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getting_position_in_sequence() {
        assert_eq!(1, get_position_in_sequence(1, 1));
        assert_eq!(2, get_position_in_sequence(2, 1));
        assert_eq!(3, get_position_in_sequence(1, 2));
        assert_eq!(4, get_position_in_sequence(3, 1));
        assert_eq!(5, get_position_in_sequence(2, 2));
        assert_eq!(6, get_position_in_sequence(1, 3));
        assert_eq!(7, get_position_in_sequence(4, 1));
        assert_eq!(21, get_position_in_sequence(1, 6));
    }

    #[test]
    fn test_example() {
        let known_codes: Vec<(usize, usize, u64)> = vec![
            (1, 1, 20151125), (1, 2, 18749137), (1, 3, 17289845),
            (2, 1, 31916031), (2, 2, 21629792), (2, 3, 16929656),
            (3, 1, 16080970), (3, 2,  8057251), (3, 3,  1601130),
        ];
        for &(row, col, expected) in known_codes.iter() {
            assert_eq!(get_code(row, col), expected);
        }
    }

    #[test]
    fn test_part_1() {
        assert_eq!(get_code(ROW, COL), 8997277);
    }

}
