extern crate itertools;

use std::collections::VecDeque;
use std::io::{self, BufRead, BufReader};

fn is_opening(c: char) -> bool {
    c == '(' || c == '<' || c == '{' || c == '['
}

fn is_matching(c: char, other: char) -> bool {
    c == '(' && other == ')'
        || c == '[' && other == ']'
        || c == '<' && other == '>'
        || c == '{' && other == '}'
}

fn is_corrupted(line: &String) -> Result<VecDeque<char>, char> {
    let mut stack = VecDeque::new();
    for c in line.chars() {
        if is_opening(c) {
            stack.push_back(c);
        } else {
            assert_ne!(stack.len(), 0);
            if is_matching(*stack.back().unwrap(), c) {
                stack.pop_back();
            } else {
                return Err(c);
            }
        }
    }
    Ok(stack)
}

fn score_char(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Invalid end character"),
    }
}

fn score_seq(seq: VecDeque<char>) -> u64 {
    let mut score = 0;
    for c in seq.iter().rev() {
        score = score * 5
            + match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("Unexpected closing character"),
            }
    }
    score
}

fn main() {
    let lines: Vec<String> = BufReader::new(io::stdin())
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    println!(
        "[Part 1] {}",
        lines
            .iter()
            .filter_map(|line| is_corrupted(&line).err())
            .map(score_char)
            .sum::<u64>()
    );
    let scores: Vec<u64> = itertools::sorted(
        lines
            .iter()
            .filter_map(|line| is_corrupted(&line).ok())
            .map(score_seq),
    )
    .collect();
    println!("[Part 2] {}", scores[scores.len() / 2]);
}
