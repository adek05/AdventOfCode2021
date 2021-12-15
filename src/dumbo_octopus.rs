extern crate itertools;

use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead, BufReader};

fn neighbors(x: i32, y: i32) -> Vec<(i32, i32)> {
    vec![
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ]
}

fn main() {
    let mut energies: Vec<Vec<u32>> = BufReader::new(io::stdin())
        .lines()
        .filter_map(|line| {
            line.map(|energies| {
                energies
                    .chars()
                    .map(|energy| energy.to_digit(10).unwrap())
                    .collect()
            })
            .ok()
        })
        .collect();
    let mut cnt = 0;
    let mut idx = 0;
    loop {
        idx += 1;
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        for i in 0..energies.len() {
            for j in 0..energies.first().unwrap().len() {
                energies[i][j] += 1;
                if energies[i][j] > 9 {
                    q.push_back((i as i32, j as i32));
                }
            }
        }
        let mut flashed: HashSet<(i32, i32)> = HashSet::new();
        while let Some((x, y)) = q.pop_front() {
            if flashed.contains(&(x, y)) {
                continue;
            }
            energies[x as usize][y as usize] += 1;
            if energies[x as usize][y as usize] > 9 {
                flashed.insert((x, y));
                energies[x as usize][y as usize] = 0;
                for n in neighbors(x, y) {
                    if n.0 < 0
                        || n.1 < 0
                        || n.0 >= energies.len() as i32
                        || n.1 >= energies.first().unwrap().len() as i32
                    {
                        continue;
                    } else {
                        q.push_back(n)
                    }
                }
            }
        }
        if idx <= 100 {
            cnt += flashed.len();
        }
        if flashed.len() == 100 {
            println!("[Part 2] {}", idx);
            break;
        }
    }
    println!("[Part 1] {}", cnt);
}
