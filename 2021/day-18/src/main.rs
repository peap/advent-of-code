use std::cell::{Cell, RefCell};
use std::fmt;
use std::ops::Add;
use std::rc::Rc;
use std::str::FromStr;

use common::{default_puzzle, Answer, BadInput, InputReader, Puzzle};

type NumWrap = Rc<RefCell<Option<Number>>>;
type Path = Vec<bool>;

#[derive(Clone, Debug)]
struct Number {
    // How far from the top-level number is this one?
    depth: Cell<u64>,

    // How do you get to this one from the top-level number?
    path: RefCell<Path>,

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
        let mut last_char_was_digit = false;
        let mut last_num_was_left = false;
        for c in string.chars() {
            match c {
                '[' => {
                    depth += 1;
                    let num = Number::new(depth, None);
                    *num.path.borrow_mut() = on_the_left.clone();
                    num_stack.push(Rc::new(RefCell::new(Some(num))));
                    on_the_left.push(true);
                    last_char_was_digit = false;
                }
                ']' => {
                    depth -= 1;
                    let finished = num_stack.pop().unwrap();
                    if let Some(current) = num_stack.pop() {
                        let otl = on_the_left.pop().unwrap();
                        if let Some(f) = &*finished.borrow() {
                            f.set_parent(current.clone());
                        }
                        if otl {
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
                    last_char_was_digit = false;
                }
                ',' => {
                    on_the_left.push(false);
                    last_char_was_digit = false;
                }
                d => {
                    let val = d.to_digit(10).unwrap() as u64;
                    let current = num_stack.pop().unwrap();
                    if last_char_was_digit {
                        if let Some(c) = &*current.borrow() {
                            if last_num_was_left {
                                if let Some(n) = &*c.left.borrow() {
                                    let new_val_str = format!("{}{}", n.get_value().unwrap(), val);
                                    let new_val = new_val_str.parse().unwrap();
                                    n.set_value(new_val);
                                }
                            } else if let Some(n) = &*c.right.borrow() {
                                let new_val_str = format!("{}{}", n.get_value().unwrap(), val);
                                let new_val = new_val_str.parse().unwrap();
                                n.set_value(new_val);
                            }
                        }
                    } else {
                        let new_num = Number::new(depth + 1, Some(val));
                        *new_num.path.borrow_mut() = on_the_left.clone();
                        new_num.set_parent(current.clone());
                        let new_num_ref = Rc::new(RefCell::new(Some(new_num)));
                        if on_the_left.pop().unwrap() {
                            last_num_was_left = true;
                            if let Some(c) = &*current.borrow() {
                                c.set_left(new_num_ref.clone());
                            }
                        } else if let Some(c) = &*current.borrow() {
                            last_num_was_left = false;
                            c.set_right(new_num_ref.clone());
                        }
                    }
                    num_stack.push(current);
                    last_char_was_digit = true;
                }
            }
        }
        let root = Rc::try_unwrap(num_stack.pop().unwrap()).unwrap();
        Ok(root.into_inner().unwrap())
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        if let Some(v) = self.get_value() {
            s.push_str(&v.to_string());
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

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let sum = Number::new(1, None);
        let sum_ref = Rc::new(RefCell::new(Some(sum)));
        if let Some(s) = &*sum_ref.borrow() {
            self.move_under_parent(sum_ref.clone(), true);
            other.move_under_parent(sum_ref.clone(), false);
            s.set_left(Rc::new(RefCell::new(Some(self))));
            s.set_right(Rc::new(RefCell::new(Some(other))));
            s.reduce();
        }
        let sum_final = Rc::try_unwrap(sum_ref).unwrap();
        sum_final.into_inner().unwrap()
    }
}

impl Number {
    fn move_under_parent(&self, parent: NumWrap, on_the_left: bool) {
        if parent.borrow().is_some() {
            self.set_parent(parent);
        }
        let dummy_parent = Rc::new(RefCell::new(None));
        self.path.borrow_mut().insert(0, on_the_left);
        let d = self.depth.get();
        self.depth.set(d + 1);
        if let Some(l) = &*self.left.borrow() {
            l.move_under_parent(dummy_parent.clone(), on_the_left);
        }
        if let Some(r) = &*self.right.borrow() {
            r.move_under_parent(dummy_parent, on_the_left);
        }
    }

    fn new(depth: u64, value: Option<u64>) -> Self {
        Number {
            depth: Cell::new(depth),
            path: RefCell::new(vec![]),
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

    fn get_root(&self) -> NumWrap {
        // Crawl up the tree. First, get a reference to ourself,
        // if we have a parent;
        let mut root: NumWrap = {
            if let Some(p) = &*self.parent.borrow() {
                let i_am_left = *self.path.borrow().last().unwrap_or(&true);
                if i_am_left {
                    p.left.clone()
                } else {
                    p.right.clone()
                }
            } else {
                // We shouldn't need to call this on the root, so it
                // should be fine to clone self in order to wrap it up.
                Rc::new(RefCell::new(Some(self.clone())))
            }
        };
        loop {
            if let Some(r) = &*(root.clone()).borrow() {
                if r.parent.borrow().is_some() {
                    root = r.parent.clone();
                } else {
                    break;
                }
            }
        }
        root
    }

    fn get_child(&self, path: Path) -> NumWrap {
        let mut child: NumWrap = if path[0] {
            self.left.clone()
        } else {
            self.right.clone()
        };
        for left in path.iter().skip(1) {
            if let Some(c) = &*(child.clone()).borrow() {
                if *left {
                    child = c.left.clone();
                } else {
                    child = c.right.clone();
                }
            }
        }
        child
    }

    fn left_to_right(&self) -> Vec<Path> {
        let mut paths = vec![];
        if let Some(l) = &*self.left.borrow() {
            if l.get_value().is_some() {
                paths.push(l.path.borrow().clone());
            }
            paths.append(&mut l.left_to_right());
        }
        if let Some(r) = &*self.right.borrow() {
            if r.get_value().is_some() {
                paths.push(r.path.borrow().clone());
            }
            paths.append(&mut r.left_to_right());
        }
        paths
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
                self.split(true);
            }
        }
    }

    fn needs_to_reduce(&self) -> bool {
        // Do we need to split?
        if self.get_value().unwrap_or(0) >= 10 {
            return true;
        }
        // Do we need to explode?
        if let Some(num) = &*self.left.borrow() {
            if num.depth.get() >= 6 || num.needs_to_reduce() {
                return true;
            }
        }
        if let Some(num) = &*self.right.borrow() {
            if num.depth.get() >= 6 || num.needs_to_reduce() {
                return true;
            }
        }
        false
    }

    fn explode(&self) -> bool {
        let mut exploded = false;
        let mut i_exploded = false;
        let mut left_val: u64 = 0;
        let mut right_val: u64 = 0;
        if self.depth.get() >= 5 && self.has_regular_pair() {
            // Try to explode this number; replace the pair with
            // zero and move its values left and right.
            {
                if let Some(l) = &*self.left.borrow() {
                    left_val = l.get_value().unwrap();
                }
                if let Some(r) = &*self.right.borrow() {
                    right_val = r.get_value().unwrap();
                }
            }
            {
                *self.left.borrow_mut() = None;
                *self.right.borrow_mut() = None;
            }
            exploded = true;
            i_exploded = true;
        } else {
            // Try to explode a child number, first on the left...
            if let Some(l) = &*self.left.borrow() {
                exploded = l.explode();
            }
            // ...then on the right.
            if !exploded {
                if let Some(r) = &*self.right.borrow() {
                    exploded = r.explode();
                }
            }
        }
        if i_exploded {
            self.set_value(0);
            if let Some(p) = &*self.parent.borrow() {
                let from = self.path.borrow().clone();
                p.move_value_left(left_val, from.clone());
                p.move_value_right(right_val, from);
            }
        }
        exploded
    }

    fn move_value_left(&self, value: u64, from: Path) {
        let mut last_path: Path = vec![];
        let mut neighbor: Option<Path> = None;
        if let Some(l) = &*self.left.borrow() {
            if l.path.borrow().clone() != from {
                if let Some(v) = l.get_value() {
                    l.set_value(v + value);
                    return;
                }
            }
        }
        if let Some(r) = &*self.get_root().borrow() {
            for (i, p) in r.left_to_right().into_iter().enumerate() {
                if i > 0 && p == from {
                    neighbor = Some(last_path);
                    break;
                }
                last_path = p.clone();
            }
            if let Some(path) = neighbor {
                if let Some(c) = &*r.get_child(path).borrow() {
                    let v = c.get_value().unwrap();
                    c.set_value(v + value);
                }
            }
        }
    }

    fn move_value_right(&self, value: u64, from: Path) {
        let mut last_path: Path = vec![];
        let mut neighbor: Option<Path> = None;
        if let Some(r) = &*self.right.borrow() {
            if r.path.borrow().clone() != from {
                if let Some(v) = r.get_value() {
                    r.set_value(v + value);
                    return;
                }
            }
        }
        if let Some(r) = &*self.get_root().borrow() {
            for p in r.left_to_right().into_iter() {
                if last_path == from {
                    neighbor = Some(p);
                    break;
                }
                last_path = p;
            }
            if let Some(path) = neighbor {
                if let Some(c) = &*r.get_child(path).borrow() {
                    let v = c.get_value().unwrap();
                    c.set_value(v + value);
                }
            }
        }
    }

    fn split(&self, i_am_left: bool) -> bool {
        let mut split = false;
        if let Some(n) = self.get_value() {
            if n >= 10 {
                let l = n / 2;
                let r = n - l;
                let left = Number::new(self.depth.get() + 1, Some(l));
                let mut left_path = self.path.borrow().clone();
                left_path.push(true);
                *left.path.borrow_mut() = left_path;
                let right = Number::new(self.depth.get() + 1, Some(r));
                let mut right_path = self.path.borrow().clone();
                right_path.push(false);
                *right.path.borrow_mut() = right_path;
                // The top-most number doesn't have a value, so by
                // the time we're spliting, we can crawl back up to
                // our parent to find an existing Rc to use here.
                let my_ref = {
                    if let Some(p) = &*self.parent.borrow() {
                        if i_am_left {
                            p.left.clone()
                        } else {
                            p.right.clone()
                        }
                    } else {
                        // Shouldn't happen :tm:
                        Rc::new(RefCell::new(None))
                    }
                };
                left.set_parent(my_ref.clone());
                right.set_parent(my_ref);
                self.value.set(None);
                self.set_left(Rc::new(RefCell::new(Some(left))));
                self.set_right(Rc::new(RefCell::new(Some(right))));
                split = true;
            }
        } else {
            // Try to split a child number, first on the left...
            if let Some(l) = &*self.left.borrow() {
                split = l.split(true);
            }
            // ...then on the right.
            if !split {
                if let Some(r) = &*self.right.borrow() {
                    split = r.split(false);
                }
            }
        }
        split
    }

    fn magnitude(&self) -> u64 {
        if let Some(v) = self.get_value() {
            return v;
        }
        let mut sum = 0;
        if let Some(num) = &*self.left.borrow() {
            sum += 3 * num.magnitude();
        }
        if let Some(num) = &*self.right.borrow() {
            sum += 2 * num.magnitude();
        }
        sum
    }
}

fn part1(reader: &InputReader) -> Answer {
    let numbers: Vec<Number> = reader.parsed_lines();
    numbers
        .into_iter()
        .reduce(|acc, a| acc + a)
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
        let num = Number::new(1, None);
        let got = Number::from_str("[]").unwrap();
        assert_eq!(got, num, "\ngot: {:#?}\nwant: {:#?}", got, num);
    }

    #[test]
    fn test_number_from_str_simple() {
        let s = "[1,2]";
        let num = Number::new(1, None);
        num.set_left(wrapped(2, Some(1)));
        num.set_right(wrapped(2, Some(2)));
        assert_eq!(num.to_string(), s);
        let got = Number::from_str(s).unwrap();
        assert_eq!(got.to_string(), s);
        assert_eq!(got, num, "\ngot: {:#?}\nwant: {:#?}", got, num);
    }

    #[test]
    fn test_number_from_str_double_digits() {
        let s = "[12,34]";
        let num = Number::new(1, None);
        num.set_left(wrapped(2, Some(12)));
        num.set_right(wrapped(2, Some(34)));
        assert_eq!(num.to_string(), s);
        let got = Number::from_str(s).unwrap();
        assert_eq!(got.to_string(), s);
        assert_eq!(got, num, "\ngot: {:#?}\nwant: {:#?}", got, num);
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
    fn test_number_left_to_right_empty() {
        let num = Number::from_str("[]").unwrap();
        let want: Vec<Path> = vec![];
        assert_eq!(num.left_to_right(), want);
    }

    #[test]
    fn test_number_left_to_right_simple() {
        let num = Number::from_str("[1,2]").unwrap();
        let want: Vec<Path> = vec![
            vec![true],  // 1
            vec![false], // 2
        ];
        assert_eq!(num.left_to_right(), want);
    }

    #[test]
    fn test_number_left_to_right_nested_right() {
        let num = Number::from_str("[1,[2,3]]").unwrap();
        let want: Vec<Path> = vec![
            vec![true],         // 1
            vec![false, true],  // 2
            vec![false, false], // 3
        ];
        assert_eq!(num.left_to_right(), want);
    }

    #[test]
    fn test_number_left_to_right_nested_left() {
        let num = Number::from_str("[[1,2],3]").unwrap();
        let want: Vec<Path> = vec![
            vec![true, true],  // 1
            vec![true, false], // 2
            vec![false],       // 3
        ];
        assert_eq!(num.left_to_right(), want);
    }

    #[test]
    fn test_number_left_to_right_bigger() {
        let num = Number::from_str("[[[[[9,8],1],2],3],4]").unwrap();
        let want: Vec<Path> = vec![
            vec![true, true, true, true, true],  // 9
            vec![true, true, true, true, false], // 8
            vec![true, true, true, false],       // 1
            vec![true, true, false],             // 2
            vec![true, false],                   // 3
            vec![false],                         // 4
        ];
        assert_eq!(num.left_to_right(), want);
        num.explode();
        // [[[[0,9],2],3],4]
        let want_exploded: Vec<Path> = vec![
            vec![true, true, true, true],  // 0
            vec![true, true, true, false], // 9
            vec![true, true, false],       // 2
            vec![true, false],             // 3
            vec![false],                   // 4
        ];
        assert_eq!(num.left_to_right(), want_exploded);
    }

    #[test]
    fn test_number_left_to_right_bigger_still() {
        let num = Number::from_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
        let want: Vec<Path> = vec![
            vec![true, true],                        // 3
            vec![true, false, true],                 // 2
            vec![true, false, false, true],          // 1
            vec![true, false, false, false, true],   // 7
            vec![true, false, false, false, false],  // 3
            vec![false, true],                       // 6
            vec![false, false, true],                // 5
            vec![false, false, false, true],         // 4
            vec![false, false, false, false, true],  // 3
            vec![false, false, false, false, false], // 2
        ];
        assert_eq!(num.left_to_right(), want);
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
    fn test_number_explode() {
        let cases = vec![
            ("[]", "[]", false),
            ("[1,2]", "[1,2]", false),
            ("[1,[2,3]]", "[1,[2,3]]", false),
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]", true),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]", true),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]", true),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                true,
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
                true,
            ),
        ];
        for (orig, want, explodes) in cases.into_iter() {
            let num = Number::from_str(orig).unwrap();
            let want = Number::from_str(want).unwrap();
            assert!(num.explode() == explodes);
            assert!(num == want);
            assert_eq!(num, want);
        }
    }

    #[test]
    fn test_number_split() {
        let cases = vec![
            ("[]", "[]", false),
            ("[1,2]", "[1,2]", false),
            ("[1,[2,3]]", "[1,[2,3]]", false),
            ("[10,0]", "[[5,5],0]", true),
            ("[0,10]", "[0,[5,5]]", true),
            ("[11,0]", "[[5,6],0]", true),
            ("[0,11]", "[0,[5,6]]", true),
            (
                "[[[[0,7],4],[15,[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
                true,
            ),
            (
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
                true,
            ),
        ];
        for (orig, want, splits) in cases.into_iter() {
            let num = Number::from_str(orig).unwrap();
            let want = Number::from_str(want).unwrap();
            assert!(num.split(true) == splits);
            assert_eq!(num, want);
        }
    }

    #[test]
    fn test_number_magnitude() {
        let cases = vec![
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
            (
                "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
                4140,
            ),
        ];
        for (num_str, want) in cases.into_iter() {
            let num = Number::from_str(num_str).unwrap();
            assert_eq!(num.magnitude(), want);
        }
    }

    #[test]
    fn test_number_reduce() {
        let num = Number::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
        num.reduce();
        assert_eq!(num.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_number_addition_simple() {
        let first = Number::from_str("[1,1]").unwrap();
        let second = Number::from_str("[2,2]").unwrap();
        let want = Number::from_str("[[1,1],[2,2]]").unwrap();
        assert_eq!(first + second, want);
    }

    #[test]
    fn test_number_addition_example() {
        let first = Number::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
        let second = Number::from_str("[1,1]").unwrap();
        let want = Number::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
        assert_eq!(first + second, want);
    }

    #[test]
    fn test_number_addition_big_example() {
        let numbers: Vec<Number> = vec![
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ]
        .into_iter()
        .map(|n| Number::from_str(n).unwrap())
        .collect();
        let reduced = numbers.into_iter().reduce(|acc, a| acc + a).unwrap();
        let want =
            Number::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap();
        assert_eq!(reduced, want);
    }

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(4137);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        get_puzzle().test_part2(0);
    }
}
