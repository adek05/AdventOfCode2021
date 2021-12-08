#[macro_use]
extern crate scan_rules;
extern crate itertools;

use scan_rules::scanner::Word;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn count_1_4_7_8(display: &[String]) -> usize {
    let lengths: HashSet<usize> = HashSet::from_iter(vec![2, 4, 3, 7]);

    display
        .iter()
        .filter(|s| lengths.contains(&s.len()))
        .count()
}

// 1 -> len 2
// 4 -> len 4
// 7 -> len 3
// 8 -> len 7
// 9 -> len 6 + zawiera 4
// 0 -> len 6 + zawiera 1 + nie zawiera 4
// 6 -> len 6 + nie zawiera 1 + nie zawiera 4
// 3 -> len 5 + zawiera 1
// 5 -> len 5 + 6 zaawiera 5
// 2 -> len 5 + 6 nie zawiera 5
fn contains(digit: &str, other: &str) -> bool {
    let d: HashSet<char> = HashSet::from_iter(digit.chars());
    d.is_superset(&HashSet::from_iter(other.chars()))
}

fn part_2(patterns: Vec<String>, display: Vec<String>) -> i32 {
    let sorted_display: Vec<String> = display
        .iter()
        .map(|p| itertools::sorted(p.chars()).collect())
        .collect();

    let digits: Vec<String> = patterns
        .iter()
        .map(|p| itertools::sorted(p.chars()).collect())
        .collect();
    let mut pattern_to_digits: HashMap<String, i32> = HashMap::new();

    let one: String = digits.iter().find(|digit| digit.len() == 2).cloned().unwrap();
    let four: String = digits.iter().find(|digit| digit.len() == 4).cloned().unwrap();
    pattern_to_digits.insert(one.clone(), 1);
    pattern_to_digits.insert(four.clone(), 4);
    pattern_to_digits.insert(digits.iter().find(|digit| digit.len() == 3).cloned().unwrap(), 7);
    pattern_to_digits.insert(digits.iter().find(|digit| digit.len() == 7).cloned().unwrap(), 8);

    pattern_to_digits.insert(
        digits
            .iter()
            .find(|digit| digit.len() == 6 && contains(*digit, &four))
            .cloned()
            .unwrap(),
        9,
    );
    pattern_to_digits.insert(
        digits
            .iter()
            .find(|digit| digit.len() == 6 && contains(*digit, &one) && !contains(*digit, &four))
            .cloned()
            .unwrap(),
        0,
    );
    let six = 
        digits
            .iter()
            .find(|digit| digit.len() == 6 && !contains(*digit, &one) && !contains(*digit, &four))
            .cloned()
            .unwrap();
    pattern_to_digits.insert(six.clone(), 6);

    pattern_to_digits.insert(
        digits
            .iter()
            .find(|digit| digit.len() == 5 && contains(*digit, &one))
            .cloned()
            .unwrap(),
        3,
    );
    pattern_to_digits.insert(
        digits
            .iter()
            .find(|digit| digit.len() == 5 && contains(&six, *digit))
            .cloned()
            .unwrap(),
        5,
    );
    pattern_to_digits.insert(
        digits
            .iter()
            .find(|digit| digit.len() == 5 && !contains(&six, *digit) && !contains(*digit, &one))
            .cloned()
            .unwrap(),
        2,
    );

    assert_eq!(pattern_to_digits.len(), 10);
    sorted_display.iter().fold(0, |acc, d| 10 * acc + pattern_to_digits[d])
}

fn main() {
    let mut res = 0;
    let mut res2 = 0;
    while let Ok((patterns, display)) = try_readln! {([let patterns: Word<String>] +, " | ", [let display: Word<String>] +) => (patterns, display)}
    {
        res += count_1_4_7_8(&display);
        res2 += part_2(patterns, display);
    }
    println!("[Part 1]: {}", res);
    println!("[Part 2]: {}", res2);
}