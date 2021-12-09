extern crate itertools;

use std::collections::VecDeque;
use std::io::{self, BufRead, BufReader};

const MAX_DEPTH: u32 = 9;

fn get_depth(map: &[Vec<u32>], x: i32, y: i32) -> u32 {
    if x < 0 || y < 0 {
        return MAX_DEPTH;
    }
    if let Some(line) = map.get(x as usize) {
        if let Some(depth) = line.get(y as usize) {
            return *depth;
        }
    }
    MAX_DEPTH
}

fn is_low_point(map: &[Vec<u32>], x: i32, y: i32) -> bool {
    let depth = get_depth(map, x, y);
    [(0, 1), (1, 0), (-1, 0), (0, -1)]
        .iter()
        .all(|(dx, dy)| get_depth(map, x + dx, y + dy) > depth)
}

fn bfs(map: &[Vec<u32>], visited: &mut Vec<Vec<bool>>, x: i32, y: i32) -> u32 {
    let mut size: u32 = 0;
    let mut queue = VecDeque::new();
    queue.push_back((x, y, get_depth(map, x, y)));
    while let Some((x, y, d)) = queue.pop_front() {
        if visited[x as usize][y as usize] {
            continue;
        }
        visited[x as usize][y as usize] = true;
        size += 1;
        for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)].iter() {
            let nd = get_depth(map, x + dx, y + dy);
            if nd > d && nd < 9 {
                queue.push_back((x + dx, y + dy, nd));
            }
        }
    }
    size
}

fn main() {
    let cave_map: Vec<Vec<u32>> = BufReader::new(io::stdin())
        .lines()
        .filter_map(|line| {
            line.map(|depths| {
                depths
                    .chars()
                    .map(|depth| depth.to_digit(10).unwrap())
                    .collect()
            })
            .ok()
        })
        .collect();
    let mut risk_score = 0;
    let mut starting_points: Vec<(i32, i32)> = vec![];
    for (i, line) in cave_map.iter().enumerate() {
        for (j, depth) in line.iter().enumerate() {
            if is_low_point(&cave_map, i as i32, j as i32) {
                starting_points.push((i as i32, j as i32));
                risk_score += depth + 1;
            }
        }
    }
    println!("[Part 1] {}", risk_score);

    let mut visited: Vec<Vec<bool>> = cave_map
        .iter()
        .cloned()
        .map(|x| x.iter().cloned().map(|_| false).collect())
        .collect();
    println!(
        "[Part 2] {}",
        itertools::sorted(
            starting_points
                .iter()
                .map(|(x, y)| bfs(&cave_map, &mut visited, *x , *y))
        )
        .rev()
        .take(3)
        .product::<u32>()
    );
}
