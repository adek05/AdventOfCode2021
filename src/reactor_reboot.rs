#[macro_use]
extern crate scan_rules;

use std::cmp::{max, min};
use std::collections::HashSet;

struct Cuboid {
    pub xs: (i32, i32),
    pub ys: (i32, i32),
    pub zs: (i32, i32),
}

enum Op {
    Off(Cuboid),
    On(Cuboid),
}

fn enumerate_cuboid(c: Cuboid) -> Vec<(i32, i32, i32)> {
    let mut res = vec![];
    for x in max(-50, c.xs.0)..min(50, c.xs.1) + 1 {
        for y in max(-50, c.ys.0)..min(50, c.ys.1) + 1 {
            for z in max(-50, c.zs.0)..min(50, c.zs.1) + 1 {
                res.push((x, y, z));
            }
        }
    }
    res
}

fn main() {
    let mut ops: Vec<Op> = Vec::new();
    while let Ok(op) = try_readln! {
       ("on x=", let x1: i32, "..", let x2:i32, ",y=", let y1: i32, "..", let y2: i32, ",z=", let z1: i32, "..", let z2: i32) => Op::On(Cuboid {xs: (x1, x2), ys: (y1, y2), zs: (z1, z2)}),
       ("off x=", let x1: i32, "..", let x2:i32, ",y=", let y1: i32, "..", let y2: i32, ",z=", let z1: i32, "..", let z2: i32) => Op::Off(Cuboid {xs: (x1, x2), ys: (y1, y2), zs: (z1, z2)}),
    } {
        ops.push(op)
    }
    assert_ne!(ops.len(), 0);
    let mut cube: HashSet<(i32, i32, i32)> = HashSet::new();

    for op in ops {
        match op {
            Op::Off(c) => enumerate_cuboid(c).iter().for_each(|p| {
                cube.remove(p);
            }),
            Op::On(c) => enumerate_cuboid(c).into_iter().for_each(|p| {
                cube.insert(p);
            }),
        }
    }

    println!("[Part 1] {}", cube.len());
}
