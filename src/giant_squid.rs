#[macro_use]
extern crate scan_rules;
extern crate itertools;

use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Board {
    board: Vec<Vec<u32>>,
    marked: Vec<Vec<bool>>,
}

impl Board {
    fn create(board: Vec<Vec<u32>>) -> Self {
        let marked: Vec<Vec<bool>> = board
            .iter()
            .clone()
            .map(|row| row.iter().map(|_| false).collect())
            .collect();
        Self { board, marked }
    }

    fn mark(&mut self, number: u32) {
        for (i, row) in self.board.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if item == &number {
                    self.marked[i][j] = true
                }
            }
        }
    }

    fn is_winning(&self) -> bool {
        // rows
        for row in self.marked.iter() {
            if row.iter().all(|x| *x) {
                return true;
            }
        }
        // columns
        for i in 0..self.marked.iter().len() {
            if self.marked.iter().map(|v| v[i]).all(|x| x) {
                return true;
            }
        }
        false
    }

    fn score(&self, mul: &u32) -> u32 {
        let mut sum = 0;
        for (i, row) in self.board.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if !self.marked[i][j] {
                    sum += item;
                }
            }
        }
        sum * mul
    }
}

fn main() {
    let numbers = readln! {([let numbers: u32],+) => numbers};

    let mut boards: Vec<(Board, Option<usize>)> = vec![];
    let mut cur: Vec<Vec<u32>> = vec![];
    for line in BufReader::new(io::stdin()).lines() {
        if let Ok(l) = line {
            if l.is_empty() {
                if !cur.is_empty() {
                    boards.push((Board::create(cur.clone()), Option::None));
                    cur = vec![];
                }
            } else {
                cur.push(scan!(&l; ([let bingo: u32] *) => bingo).unwrap());
            }
        }
    }

    for (i, number) in numbers.iter().enumerate() {
        boards = boards
            .into_iter()
            .map(|(mut board, x)| {
                if x.is_none() {
                board.mark(*number);
                }
                let r = if board.is_winning() {
                    x.or(Option::Some(i))
                } else {
                    Option::None
                };
                (board, r)
            })
            .collect();
    }
    if let Some((board, Some(idx))) = boards.iter().min_by_key(|(_board, won_at)| won_at) {
        println!("[Part 1] {}", board.score(&numbers[*idx]));
    }
    if let Some((board, Some(idx))) = boards.iter().max_by_key(|(_board, won_at)| won_at) {
        println!("[Part 2] {}", board.score(&numbers[*idx]));
    }
}
