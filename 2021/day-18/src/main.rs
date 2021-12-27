use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use common::{default_puzzle, Answer, BadInput, InputReader, Puzzle};

#[derive(Clone, Debug)]
struct Number {
    // How far from the top-level number is this one?
    depth: u64,

    parent: RefCell<Weak<Number>>,

    // A `Number` is a pair, where the left or right member can be a
    // simple number or another pair. The simplest number is [x,y],
    // where the top-level number has `value=None`, `left` is another
    // `Number` with `value=Some(x)` and right is a third `Number`
    // with `value=Some(y)`.
    value: Option<u64>,
    left: RefCell<Option<Rc<Number>>>,
    right: RefCell<Option<Rc<Number>>>,
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        vec![
            self.depth == other.depth,
            self.value == other.value,
            self.left == other.left,
            self.right == other.right,
        ]
        .iter()
        .all(|&c| c)
    }
}

impl FromStr for Number {
    type Err = BadInput;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut num_stack: Vec<Rc<Number>> = vec![];
        let mut on_the_left: Vec<bool> = vec![];
        let mut depth = 0;
        for c in string.chars() {
            match c {
                '[' => {
                    depth += 1;
                    on_the_left.push(true);
                    num_stack.push(Rc::new(Number::new(depth, None)));
                }
                ']' => {
                    depth -= 1;
                    let finished = num_stack.pop().unwrap();
                    if let Some(current) = num_stack.pop() {
                        finished.set_parent(&current);
                        if on_the_left.pop().unwrap() {
                            current.set_left(finished);
                        } else {
                            current.set_right(finished);
                        }
                        num_stack.push(current);
                    } else {
                        num_stack.push(finished);
                    }
                }
                ',' => {
                    on_the_left.push(false);
                }
                d => {
                    let val = d.to_digit(10).unwrap() as u64;
                    let current = num_stack.pop().unwrap();
                    let mut new_num = Number::new(depth + 1, None);
                    new_num.set_parent(&current);
                    new_num.set_value(val);
                    if on_the_left.pop().unwrap() {
                        current.set_left(Rc::new(new_num));
                    } else {
                        current.set_right(Rc::new(new_num));
                    }
                    num_stack.push(current);
                }
            }
        }
        if num_stack.len() != 1 {
            panic!("Eric did it wrong");
        }
        Rc::try_unwrap(num_stack.pop().unwrap()).or(Err(BadInput))
    }
}

impl Number {
    fn new(depth: u64, value: Option<u64>) -> Self {
        Number {
            depth,
            parent: RefCell::new(Weak::new()),
            value,
            left: RefCell::new(None),
            right: RefCell::new(None),
        }
    }

    fn set_parent(&self, parent: &Rc<Number>) {
        *self.parent.borrow_mut() = Rc::downgrade(parent);
    }

    fn get_value(&self) -> Option<u64> {
        self.value
    }

    fn set_value(&mut self, value: u64) {
        self.value = Some(value);
    }

    fn set_left(&self, num: Rc<Number>) {
        *self.left.borrow_mut() = Some(num);
    }

    fn set_right(&self, num: Rc<Number>) {
        *self.right.borrow_mut() = Some(num);
    }

    fn magnitude(&self) -> u64 {
        0
    }

    fn has_regular_pair(&self) -> bool {
        let left = self.left.borrow();
        let right = self.right.borrow();
        if let Some(l) = &*left {
            if let Some(r) = &*right {
                l.get_value().is_some() && r.get_value().is_some()
            } else {
                false
            }
        } else {
            false
        }
    }

    fn reduce(&mut self) {
        while self.needs_to_reduce() {
            if !self.explode() {
                self.split();
            }
        }
    }

    fn needs_to_reduce(&self) -> bool {
        // Do we need to split?
        if self.get_value().unwrap_or(0) > 10 {
            return true;
        }
        // Do we need to explode?
        if let Some(num) = &*self.left.borrow() {
            if num.depth >= 4 || num.needs_to_reduce() {
                return true;
            }
        }
        if let Some(num) = &*self.right.borrow() {
            if num.depth >= 4 || num.needs_to_reduce() {
                return true;
            }
        }
        false
    }

    fn explode(&mut self) -> bool {
        println!("vvvvvvvvvvvv");
        println!(" d={},v={:?}", self.depth, self.value);
        println!("   l={:?}", self.left);
        println!("   r={:?}", self.right);
        let mut exploded = false;
        let mut left_val: u64 = 0;
        let mut right_val: u64 = 0;
        if self.depth >= 4 && self.has_regular_pair() {
            // Try to explode this number.
            let left = self.left.borrow();
            let right = self.right.borrow();
            // Replace the pair with zero and move its values left
            // and right.
            if let Some(l) = &*left {
                left_val = l.get_value().unwrap();
            }
            println!("  left: {}", left_val);
            if let Some(r) = &*right {
                right_val = r.get_value().unwrap();
            }
            println!(" right: {}", right_val);
            *self.left.borrow_mut() = None;
            *self.right.borrow_mut() = None;
            self.value = Some(0);
            exploded = true;
        } else {
            // Try to explode a child number, first on the left...
            println!(" explode left...");
            let mut left = self.left.borrow_mut();
            if let Some(l) = left.as_mut() {
                println!("  ...have mut borrow...");
                // TODO: get_mut doesn't work when there are other
                // references; we probably need to switch the order
                // of Rc and RefCell in the Number struct :/.
                if let Some(n) = Rc::get_mut(l) {
                    println!("   ...let's go");
                    exploded = n.explode();
                }
            }
            // ...then on the right.
            println!(" explode right...");
            let mut right = self.right.borrow_mut();
            if !exploded && right.is_some() {
                if let Some(r) = right.as_mut() {
                    println!("  ...have mut borrow...");
                    if let Some(n) = Rc::get_mut(r) {
                        println!("  ...let's go");
                        exploded = n.explode();
                    }
                }
            }
        }
        if exploded {
            self.move_value_left(left_val);
            self.move_value_right(right_val);
        }
        println!("^^^^^^^^^^^^");
        exploded
    }

    fn move_value_left(&mut self, value: u64) {
        let mut left = self.left.borrow_mut();
        if let Some(l) = left.as_mut() {
            if let Some(v) = l.get_value() {
                Rc::get_mut(l).unwrap().set_value(v + value);
            }
        } else {
            let mut parent = self.parent.borrow_mut().upgrade();
            if let Some(p) = parent.as_mut() {
                Rc::get_mut(p).unwrap().move_value_left(value);
            }
        }
    }

    fn move_value_right(&mut self, value: u64) {
        let mut right = self.right.borrow_mut();
        if let Some(r) = right.as_mut() {
            if let Some(v) = r.get_value() {
                Rc::get_mut(r).unwrap().set_value(v + value);
            }
        } else {
            let mut parent = self.parent.borrow_mut().upgrade();
            if let Some(p) = parent.as_mut() {
                Rc::get_mut(p).unwrap().move_value_right(value);
            }
        }
    }

    fn split(&mut self) -> bool {
        false
    }
}

fn part1(reader: &InputReader) -> Answer {
    let numbers: Vec<Number> = reader.parsed_lines();
    numbers
        .into_iter()
        .reduce(|acc, a| {
            let left_num = acc;
            let right_num = a;
            let mut sum = Number::new(0, None);
            sum.set_left(Rc::new(left_num));
            sum.set_right(Rc::new(right_num));
            sum.reduce();
            sum
        })
        .unwrap()
        .magnitude()
}

fn part2(reader: &InputReader) -> Answer {
    let _lines: Vec<String> = reader.parsed_lines();
    0
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Snailfish");
    puzzle.set_part1(part1, "magnitude of final sum");
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
    fn test_number_from_str_empty() {
        let num = Rc::new(Number::new(1, None));
        let got = Number::from_str("[]").unwrap();
        assert_eq!(got, *num, "\ngot: {:#?}\nwant: {:#?}", got, num);
    }

    #[test]
    fn test_number_from_str_simple() {
        let num = Rc::new(Number::new(1, None));
        num.set_left(Rc::new(Number::new(2, Some(1))));
        num.set_right(Rc::new(Number::new(2, Some(2))));
        let got = Number::from_str("[1,2]").unwrap();
        assert_eq!(got, *num, "\ngot: {:#?}\nwant: {:#?}", got, num);
    }

    #[test]
    fn test_number_from_str_nested() {
        let num = Rc::new(Number::new(1, None));
        num.set_left(Rc::new(Number::new(2, Some(1))));
        let l2 = Rc::new(Number::new(2, None));
        l2.set_parent(&num);
        l2.set_left(Rc::new(Number::new(3, Some(2))));
        l2.set_right(Rc::new(Number::new(3, Some(3))));
        num.set_right(l2);
        let got = Number::from_str("[1,[2,3]]").unwrap();
        assert_eq!(got, *num, "\ngot: {:#?}\nwant: {:#?}", got, num);
    }

    #[test]
    fn test_number_from_str_nested_deep_left() {
        let num = Rc::new(Number::new(1, None));
        num.set_left(Rc::new(Number::new(2, Some(1))));
        let l2 = Rc::new(Number::new(2, None));
        l2.set_left(Rc::new(Number::new(3, Some(2))));
        let l3 = Rc::new(Number::new(3, None));
        l3.set_left(Rc::new(Number::new(4, Some(3))));
        l3.set_right(Rc::new(Number::new(4, Some(4))));
        l2.set_right(l3);
        num.set_right(l2);
        let got = Number::from_str("[1,[2,[3,4]]]").unwrap();
        assert_eq!(got, *num, "\ngot: {:#?}\nwant: {:#?}", got, num);
    }

    #[test]
    fn test_number_from_str_nested_deep_right() {
        let num = Rc::new(Number::new(1, None));
        num.set_right(Rc::new(Number::new(2, Some(4))));
        let l2 = Rc::new(Number::new(2, None));
        l2.set_right(Rc::new(Number::new(3, Some(3))));
        let l3 = Rc::new(Number::new(3, None));
        l3.set_right(Rc::new(Number::new(4, Some(2))));
        l3.set_left(Rc::new(Number::new(4, Some(1))));
        l2.set_left(l3);
        num.set_left(l2);
        let got = Number::from_str("[[[1,2],3],4]").unwrap();
        assert_eq!(got, *num, "\ngot: {:#?}\nwant: {:#?}", got, num);
    }

    #[test]
    fn test_number_has_regular_pair() {
        // Empty number.
        let empty = Number::new(1, None);
        assert!(!empty.has_regular_pair());
        // Simple number.
        let one_two = empty.clone();
        one_two.set_left(Rc::new(Number::new(2, Some(1))));
        one_two.set_right(Rc::new(Number::new(2, Some(2))));
        assert!(one_two.has_regular_pair());
        // One nested number, on the left.
        let one_one_two = empty.clone();
        one_one_two.set_left(Rc::new(Number::new(2, Some(1))));
        let mut one_two_deeper = empty.clone();
        one_two_deeper.depth = 2;
        one_two_deeper.set_left(Rc::new(Number::new(3, Some(1))));
        one_two_deeper.set_right(Rc::new(Number::new(3, Some(2))));
        one_one_two.set_right(Rc::new(one_two_deeper.clone()));
        assert!(!one_one_two.has_regular_pair());
        assert!(one_two_deeper.has_regular_pair());
    }

    #[test]
    fn test_number_explosion() {
        // Explode examples.
        let mut num = Number::from_str("[[[[[9,8],1],2],3],4]").unwrap();
        assert!(num.explode());
        let exploded = Number::from_str("[[[[0,9],2],3],4]").unwrap();
        assert_eq!(num, exploded, " got: {:#?}\nwant: {:#?}", num, exploded);
    }

    #[test]
    #[ignore]
    fn test_part1() {
        get_puzzle().test_part1(0);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        get_puzzle().test_part2(0);
    }
}
