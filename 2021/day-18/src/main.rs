use std::cell::{Cell, RefCell};
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;

use common::{default_puzzle, Answer, BadInput, InputReader, Puzzle};

type NumWrap = Rc<RefCell<Option<Number>>>;

#[derive(Clone, Debug)]
struct Number {
    // How far from the top-level number is this one?
    depth: u64,

    // If we don't have a left or a right node, we have a value.
    value: Cell<Option<u64>>,

    // A `Number` is a pair, where the left or right member can be a
    // simple number or another pair. The simplest number is [x,y],
    // where the top-level number has `value=None`, `left` is another
    // `Number` with `value=Some(x)` and right is a third `Number`
    // with `value=Some(y)`.
    parent: NumWrap,
    left: NumWrap,
    right: NumWrap,
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl FromStr for Number {
    type Err = BadInput;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut num_stack: Vec<NumWrap> = vec![];
        let mut on_the_left: Vec<bool> = vec![];
        let mut depth = 0;
        for c in string.chars() {
            match c {
                '[' => {
                    depth += 1;
                    on_the_left.push(true);
                    let num = Number::new(depth, None);
                    num_stack.push(Rc::new(RefCell::new(Some(num))));
                }
                ']' => {
                    depth -= 1;
                    let finished = num_stack.pop().unwrap();
                    if let Some(current) = num_stack.pop() {
                        if let Some(f) = &*finished.borrow() {
                            f.set_parent(current.clone());
                        }
                        if on_the_left.pop().unwrap() {
                            if let Some(c) = &*current.borrow() {
                                c.set_left(finished);
                            }
                        } else if let Some(c) = &*current.borrow() {
                            c.set_right(finished);
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
                    let new_num = Number::new(depth + 1, Some(val));
                    new_num.set_parent(current.clone());
                    let new_num_ref = Rc::new(RefCell::new(Some(new_num)));
                    if on_the_left.pop().unwrap() {
                        if let Some(c) = &*current.borrow() {
                            c.set_left(new_num_ref);
                        }
                    } else if let Some(c) = &*current.borrow() {
                        c.set_right(new_num_ref);
                    }
                    num_stack.push(current);
                }
            }
        }
        if num_stack.len() != 1 {
            panic!("Eric did it wrong");
        }
        let root = Rc::try_unwrap(num_stack.pop().unwrap()).unwrap();
        Ok(root.into_inner().unwrap())
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        if let Some(v) = self.value.get() {
            s.push_str(&format!("{}", v));
        } else {
            s.push('[');
        }
        if let Some(l) = &*self.left.borrow() {
            s.push_str(&l.to_string());
            s.push(',');
        }
        if let Some(r) = &*self.right.borrow() {
            s.push_str(&r.to_string());
        }
        if self.value.get().is_none() {
            s.push(']');
        }
        write!(f, "{}", s)
    }
}

impl Number {
    fn new(depth: u64, value: Option<u64>) -> Self {
        Number {
            depth,
            value: Cell::new(value),
            parent: Rc::new(RefCell::new(None)),
            left: Rc::new(RefCell::new(None)),
            right: Rc::new(RefCell::new(None)),
        }
    }

    fn get_value(&self) -> Option<u64> {
        self.value.get()
    }

    fn set_value(&self, value: u64) {
        self.value.set(Some(value));
    }

    fn set_parent(&self, parent: NumWrap) {
        self.parent.replace(parent.borrow().clone());
    }

    fn set_left(&self, num: NumWrap) {
        self.left.replace(num.borrow().clone());
    }

    fn set_right(&self, num: NumWrap) {
        self.right.replace(num.borrow().clone());
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

    fn reduce(&self) {
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

    fn explode(&self) -> bool {
        println!("vvvvvvvvvvvv");
        println!(" d={},v={:?}", self.depth, self.value.get());
        let mut exploded = false;
        let mut i_exploded = false;
        let mut left_val: u64 = 0;
        let mut right_val: u64 = 0;
        if self.depth >= 4 && self.has_regular_pair() {
            // Try to explode this number; replace the pair with
            // zero and move its values left and right.
            {
                if let Some(l) = &*self.left.borrow() {
                    left_val = l.get_value().unwrap();
                }
                println!("  left: {}", left_val);
                if let Some(r) = &*self.right.borrow() {
                    right_val = r.get_value().unwrap();
                }
                println!(" right: {}", right_val);
            }
            {
                *self.left.borrow_mut() = None;
                *self.right.borrow_mut() = None;
            }
            // self.set_value(0);
            exploded = true;
            i_exploded = true;
        } else {
            // Try to explode a child number, first on the left...
            if let Some(l) = &*self.left.borrow() {
                println!(" explode left...");
                exploded = l.explode();
            }
            // ...then on the right.
            if !exploded {
                if let Some(r) = &*self.right.borrow() {
                    println!(" explode right...");
                    exploded = r.explode();
                }
            }
        }
        if i_exploded {
            if let Some(p) = &*self.parent.borrow() {
                p.move_value_left(left_val);
                p.move_value_right(right_val);
            }
            self.set_value(0);
        }
        println!("^^^^^^^^^^^^");
        exploded
    }

    fn move_value_left(&self, value: u64) {
        println!("  move value left...");
        println!("   d={},v={:?}", self.depth, self.value.get());
        if let Some(l) = &*self.left.borrow() {
            if let Some(v) = l.get_value() {
                println!("   value moved ({})", value);
                l.set_value(v + value);
                return;
            }
        }
        if let Some(p) = &*self.parent.borrow() {
            println!("   try parent...");
            p.move_value_left(value);
        }
    }

    fn move_value_right(&self, value: u64) {
        println!("  move value right...");
        println!("   d={},v={:?}", self.depth, self.value.get());
        if let Some(r) = &*self.right.borrow() {
            if let Some(v) = r.get_value() {
                println!("   value moved ({})", value);
                r.set_value(v + value);
                return;
            }
        }
        if let Some(p) = &*self.parent.borrow() {
            println!("   try parent...");
            p.move_value_right(value);
        }
    }

    fn split(&self) -> bool {
        // TODO
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
            let sum = Number::new(0, None);
            sum.set_left(Rc::new(RefCell::new(Some(left_num))));
            sum.set_right(Rc::new(RefCell::new(Some(right_num))));
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

    fn wrapped(depth: u64, value: Option<u64>) -> NumWrap {
        Rc::new(RefCell::new(Some(Number::new(depth, value))))
    }

    fn set_left(num: NumWrap, left: NumWrap) {
        if let Some(n) = &*num.borrow() {
            n.set_left(left);
        }
    }

    fn set_right(num: NumWrap, right: NumWrap) {
        if let Some(n) = &*num.borrow() {
            n.set_right(right);
        }
    }

    fn set_parent(num: NumWrap, parent: NumWrap) {
        if let Some(n) = &*num.borrow() {
            n.set_parent(parent);
        }
    }

    #[test]
    fn test_number_from_str_empty() {
        let num = Rc::new(Number::new(1, None));
        let got = Number::from_str("[]").unwrap();
        assert_eq!(got, *num, "\ngot: {:#?}\nwant: {:#?}", got, num);
    }

    #[test]
    fn test_number_from_str_simple() {
        let s = "[1,2]";
        let num = Rc::new(Number::new(1, None));
        num.set_left(wrapped(2, Some(1)));
        num.set_right(wrapped(2, Some(2)));
        assert_eq!(num.to_string(), s);
        let got = Number::from_str(s).unwrap();
        assert_eq!(got.to_string(), s);
        assert_eq!(got, *num, "\ngot: {:#?}\nwant: {:#?}", got, num);
    }

    #[test]
    fn test_number_from_str_nested() {
        let s = "[1,[2,3]]";
        let num = wrapped(1, None);
        set_left(num.clone(), wrapped(2, Some(1)));
        let l2 = wrapped(2, None);
        set_parent(l2.clone(), num.clone());
        set_left(l2.clone(), wrapped(3, Some(2)));
        set_right(l2.clone(), wrapped(3, Some(3)));
        set_right(num.clone(), l2.clone());
        let got = Number::from_str(s).unwrap();
        assert_eq!(got.to_string(), s);
        if let Some(n) = &*num.borrow() {
            assert_eq!(n.to_string(), s);
            assert_eq!(got, *n, "\ngot: {:#?}\nwant: {:#?}", got, n);
        } else {
            panic!("Nothing to borrow");
        };
    }

    #[test]
    fn test_number_from_str_nested_deep_left() {
        let num = wrapped(1, None);
        set_left(num.clone(), wrapped(2, Some(1)));
        let l2 = wrapped(2, None);
        set_left(l2.clone(), wrapped(3, Some(2)));
        let l3 = wrapped(3, None);
        set_left(l3.clone(), wrapped(4, Some(3)));
        set_right(l3.clone(), wrapped(4, Some(4)));
        set_right(l2.clone(), l3.clone());
        set_right(num.clone(), l2.clone());
        let got = Number::from_str("[1,[2,[3,4]]]").unwrap();
        if let Some(n) = &*num.borrow() {
            assert_eq!(got, *n, "\ngot: {:#?}\nwant: {:#?}", got, n);
        } else {
            panic!("Nothing to borrow");
        };
    }

    #[test]
    fn test_number_from_str_nested_deep_right() {
        let num = wrapped(1, None);
        set_right(num.clone(), wrapped(2, Some(4)));
        let l2 = wrapped(2, None);
        set_right(l2.clone(), wrapped(3, Some(3)));
        let l3 = wrapped(3, None);
        set_right(l3.clone(), wrapped(4, Some(2)));
        set_left(l3.clone(), wrapped(4, Some(1)));
        set_left(l2.clone(), l3.clone());
        set_left(num.clone(), l2.clone());
        let got = Number::from_str("[[[1,2],3],4]").unwrap();
        if let Some(n) = &*num.borrow() {
            assert_eq!(got, *n, "\ngot: {:#?}\nwant: {:#?}", got, n);
        } else {
            panic!("Nothing to borrow");
        };
    }

    #[test]
    fn test_number_to_string() {
        let strs = vec![
            "[]",
            "[1,2]",
            "[1,[2,3]]",
            "[[1,2],3]",
            "[[1,2],[3,4]]",
            "[[[1,2],3],4]",
            "[7,[6,[5,[4,[3,2]]]]]",
            "[7,[6,[5,[7,0]]]]",
        ];
        for s in strs.into_iter() {
            let num = Number::from_str(s).unwrap();
            assert_eq!(num.to_string(), s);
        }
    }

    #[test]
    fn test_number_has_regular_pair_empty() {
        let num = Number::from_str("[]").unwrap();
        assert!(!num.has_regular_pair());
    }

    #[test]
    fn test_number_has_regular_pair_simple() {
        let num = Number::from_str("[1,2]").unwrap();
        assert!(num.has_regular_pair());
    }

    #[test]
    fn test_number_has_regular_pair_nested() {
        // One nested number, on the left.
        let num = Number::from_str("[1,[2,3]]").unwrap();
        assert!(!num.has_regular_pair());
        if let Some(l) = &*num.left.borrow() {
            assert!(!l.has_regular_pair());
        };
        if let Some(r) = &*num.right.borrow() {
            assert!(r.has_regular_pair());
        };
    }

    #[test]
    fn test_number_explode_empty() {
        let num = Number::from_str("[]").unwrap();
        let want = Number::from_str("[]").unwrap();
        assert!(!num.explode());
        assert_eq!(num, want);
    }

    #[test]
    fn test_number_explode_simple() {
        let num = Number::from_str("[1,2]").unwrap();
        let want = Number::from_str("[1,2]").unwrap();
        assert!(!num.explode());
        assert_eq!(num, want);
    }

    #[test]
    fn test_number_explode_nested() {
        let num = Number::from_str("[1,[2,3]]").unwrap();
        let want = Number::from_str("[1,[2,3]]").unwrap();
        assert!(!num.explode());
        assert_eq!(num, want);
    }

    #[test]
    fn test_number_explode_example1() {
        let num = Number::from_str("[[[[[9,8],1],2],3],4]").unwrap();
        let want = Number::from_str("[[[[0,9],2],3],4]").unwrap();
        assert!(num != want);
        assert!(num.explode());
        assert_eq!(num, want);
    }

    #[test]
    fn test_number_explode_example2() {
        let num = Number::from_str("[7,[6,[5,[4,[3,2]]]]]").unwrap();
        let want = Number::from_str("[7,[6,[5,[7,0]]]]").unwrap();
        assert!(num != want);
        assert!(num.explode());
        assert_eq!(num, want);
    }

    #[test]
    fn test_number_explode_example3() {
        let num = Number::from_str("[[6,[5,[4,[3,2]]]],1]").unwrap();
        let want = Number::from_str("[[6,[5,[7,0]]],3]").unwrap();
        assert!(num != want);
        assert!(num.explode());
        assert_eq!(num, want);
    }

    #[test]
    #[ignore]
    fn test_number_explode_example4() {
        let num = Number::from_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
        let want = Number::from_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap();
        assert!(num != want);
        assert!(num.explode());
        // assert!(num == want);
        // assert_eq!(num, want);
    }

    #[test]
    #[ignore]
    fn test_number_explode_example5() {
        let num = Number::from_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap();
        let want = Number::from_str("[[3,[2,[8,0]]],[9,[5,[7,0]]]]").unwrap();
        assert!(num != want);
        assert!(num.explode());
        // assert!(num == want);
        // assert_eq!(num, want);
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
