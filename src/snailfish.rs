#![feature(iterator_fold_self)]
use std::boxed::Box;
use std::io::{self, BufRead, BufReader};
use std::iter::{Iterator, Peekable};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Number {
    Value(i64),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    fn print(&self) -> String {
        match self {
            Number::Value(x) => format!("{}", x),
            Number::Pair(l, r) => format!("[{},{}]", l.print(), r.print()),
        }
    }
}

fn parse_value<I>(it: &mut Peekable<I>) -> Option<Number>
where
    I: Iterator<Item = char>,
{
    let mut number: i64 = 0;
    let mut n_read: usize = 0;
    while it.peek().unwrap() != &',' && it.peek().unwrap() != &']' {
        number = 10 * number + ((it.next().unwrap() as u8) - ('0' as u8)) as i64;
        n_read += 1;
    }
    if n_read > 0 {
        Some(Number::Value(number))
    } else {
        None
    }
}

fn parse_pair<I>(it: &mut Peekable<I>) -> Option<Number>
where
    I: Iterator<Item = char>,
{
    if it.peek().unwrap() == &'[' {
        it.next();
        let n1 = parse_number(it).unwrap();
        assert_eq!(it.next().unwrap(), ',');
        let n2 = parse_number(it).unwrap();
        assert_eq!(it.next().unwrap(), ']');
        return Some(Number::Pair(Box::new(n1), Box::new(n2)));
    }
    None
}

fn parse_number<I>(it: &mut Peekable<I>) -> Option<Number>
where
    I: Iterator<Item = char>,
{
    parse_pair(it).or_else(|| parse_value(it))
}

fn explode(n: Number) -> Number {
    explode_impl(n, 1).0
}

fn add_to_first_left(n: Number, val: Number) -> Option<Number> {
    match (n, val) {
        (Number::Value(x), Number::Value(val)) => Some(Number::Value(x + val)),
        (Number::Pair(l, r), Number::Value(val)) => Some(Number::Pair(
            Box::new(add_to_first_left(*l, Number::Value(val)).unwrap()),
            r,
        )),
        (x, val) => panic!(
            "add_to_first_left_failed. Failed to add {:?} to tree {:?}",
            val, x
        ),
    }
}

fn add_to_first_right(n: Number, val: Number) -> Option<Number> {
    match (n, val) {
        (Number::Value(x), Number::Value(val)) => Some(Number::Value(x + val)),
        (Number::Pair(l, r), Number::Value(val)) => Some(Number::Pair(
            l,
            Box::new(add_to_first_right(*r, Number::Value(val)).unwrap()),
        )),
        x => panic!("add_to_first_right_failed {:?}", x),
    }
}

fn explode_impl(n: Number, depth: usize) -> (Number, Option<Number>, Option<Number>) {
    if depth == 5 {
        match n {
            Number::Value(v) => (Number::Value(v), None, None),
            Number::Pair(l, r) => (Number::Value(0), Some(*l), Some(*r)),
        }
    } else {
        match n {
            Number::Value(v) => (Number::Value(v), None, None),
            Number::Pair(l, r) => {
                let (new_l, add_to_left_tree, add_to_right_tree) =
                    explode_impl(*l.clone(), depth + 1);
                if let Some(rr) = add_to_right_tree {
                    if let Some(new_r) = add_to_first_left(*r, rr) {
                        (
                            Number::Pair(Box::new(new_l), Box::new(new_r)),
                            add_to_left_tree,
                            None,
                        )
                    } else {
                        panic!("Broken number, can't add to regular number in r");
                    }
                } else if add_to_left_tree.is_some() {
                    assert_eq!(add_to_right_tree, None);
                    (
                        Number::Pair(Box::new(new_l), r),
                        add_to_left_tree,
                        add_to_right_tree,
                    )
                } else if *l != new_l {
                    (
                        Number::Pair(Box::new(new_l), r),
                        add_to_left_tree,
                        add_to_right_tree,
                    )
                } else {
                    let (new_r, add_to_left_tree, add_to_right_tree) =
                        explode_impl(*r.clone(), depth + 1);
                    if let Some(ll) = add_to_left_tree {
                        if let Some(new_l) = add_to_first_right(*l, ll) {
                            return (
                                Number::Pair(Box::new(new_l), Box::new(new_r)),
                                None,
                                add_to_right_tree,
                            );
                        } else {
                            panic!("Broken number, can't add to regular number in r");
                        }
                    } else if add_to_right_tree.is_some() {
                        assert_eq!(add_to_left_tree, None);
                        return (
                            Number::Pair(Box::new(new_l), Box::new(new_r)),
                            add_to_left_tree,
                            add_to_right_tree,
                        );
                    }
                    (
                        Number::Pair(Box::new(new_l), Box::new(new_r)),
                        add_to_left_tree,
                        add_to_right_tree,
                    )
                }
            }
        }
    }
}

fn split(n: &Number) -> Number {
    match n {
        Number::Value(n) => {
            if n >= &10 {
                Number::Pair(
                    Box::new(Number::Value(n / 2)),
                    Box::new(Number::Value((n + 1) / 2)),
                )
            } else {
                Number::Value(*n)
            }
        }
        Number::Pair(l, r) => {
            let new_l = split(l);
            if new_l != **l {
                return Number::Pair(Box::new(new_l), r.clone());
            }
            Number::Pair(Box::new(new_l), Box::new(split(r)))
        },
    }
}

fn reduce(mut n: Number) -> Number {
    let mut new_n: Number;
    loop {
        new_n = explode(n.clone());
        if new_n != n {
            n = new_n;
            continue;
        }
        new_n = split(&n);
        if new_n != n {
            n = new_n;
            continue;
        }
        break;
    }
    new_n
}

fn add(l: Number, r: Number) -> Number {
    reduce(Number::Pair(Box::new(l), Box::new(r)))
}

fn magnitude(n: Number) -> i64 {
    match n {
        Number::Value(n) => n,
        Number::Pair(l, r) => 3 * magnitude(*l) + 2* magnitude(*r),
    }
}

fn main() {
    let numbers: Vec<Number> = BufReader::new(io::stdin())
        .lines()
        .filter_map(|l| parse_number(&mut l.unwrap().chars().peekable()))
        .collect();

    let res = reduce(numbers.iter().cloned().fold_first(add).unwrap());
    println!("[Part 1] {}", magnitude(res));

    let mut max_magnitude = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                max_magnitude = std::cmp::max(max_magnitude, magnitude(add(numbers[i].clone(), numbers[j].clone())));
            }
        }
    }
    println!("[Part 2] {}", max_magnitude)
}
