#[macro_use]
extern crate scan_rules;

use scan_rules::scanner::Word;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    pub x: usize,
    pub y: usize,
}

fn neighbors(p: &Coord, xlen: usize, ylen: usize) -> Vec<Coord> {
    let points = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .iter()
        .map(|(dx, dy)| (p.x as i32 + dx, p.y as i32 + dy));
    let mut res: Vec<Coord> = vec![];
    for (x, y) in points {
        if x >= 0 && x < xlen as i32 && y >= 0 && y < ylen as i32 {
            res.push(Coord {
                x: x as usize,
                y: y as usize,
            })
        }
    }
    res
}

fn getDist(p: &Coord, cavern: &Vec<Vec<usize>>, xlen: usize, ylen: usize) -> usize {
    let risk = cavern[p.x % xlen][p.y % ylen] + p.x / xlen + p.y / xlen;
    risk % 10 + risk / 10
}

fn main() {
    assert_eq!((1, Coord { x: 1, y: 2 }) < (2, Coord { x: 1, y: 2 }), true);

    let mut cavern: Vec<Vec<usize>> = vec![];
    let mut ylen: usize = 0;
    while let Ok(line) = try_readln! { (let l: Word<String>) => l } {
        let l: Vec<usize> = line
            .chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect();
        ylen = l.len();
        cavern.push(l);
    }
    let xlen = cavern.len();

    let mut visited: HashSet<Coord> = HashSet::new();
    let mut h: BinaryHeap<Reverse<(usize, Coord)>> = BinaryHeap::new();
    h.push(Reverse((0, Coord { x: 0, y: 0 })));

    while let Some(Reverse((d, coord))) = h.pop() {
        if visited.contains(&coord) {
            continue;
        }
        if coord.x + 1 == 5 * xlen && coord.y + 1 == 5 * ylen {
            println!("[Part 1] {}", d);
        }
        for neighbor in neighbors(&coord, 5 * xlen, 5 * ylen) {
            h.push(Reverse((
                d + getDist(&neighbor, &cavern, xlen, ylen),
                neighbor,
            )));
        }

        visited.insert(coord);
    }
}
