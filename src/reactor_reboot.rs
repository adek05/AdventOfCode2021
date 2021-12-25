#[macro_use]
extern crate scan_rules;

use std::cmp::{max, min};
use std::collections::HashSet;

#[derive(Clone)]
struct Cuboid {
    pub xs: (i64, i64),
    pub ys: (i64, i64),
    pub zs: (i64, i64),
}

enum Op {
    Off(Cuboid),
    On(Cuboid),
}

fn intersect(c: &Cuboid, d: &Cuboid) -> Option<Cuboid> {
    let xs = (max(c.xs.0, d.xs.0), min(c.xs.1, d.xs.1));
    let ys = (max(c.ys.0, d.ys.0), min(c.ys.1, d.ys.1));
    let zs = (max(c.zs.0, d.zs.0), min(c.zs.1, d.zs.1));

    if xs.0 <= xs.1 && ys.0 <= ys.1 && zs.0 <= zs.1 {
        return Some(Cuboid { xs, ys, zs });
    }
    None
}

fn count_points(c: &Cuboid) -> i64 {
    (c.xs.1 - c.xs.0 + 1) * (c.ys.1 - c.ys.0 + 1) * (c.zs.1 - c.zs.0 + 1)
}

fn main() {
    let mut ops: Vec<Op> = Vec::new();
    while let Ok(op) = try_readln! {
       ("on x=", let x1: i64, "..", let x2:i64, ",y=", let y1: i64, "..", let y2: i64, ",z=", let z1: i64, "..", let z2: i64) => Op::On(Cuboid {xs: (x1, x2), ys: (y1, y2), zs: (z1, z2)}),
       ("off x=", let x1: i64, "..", let x2:i64, ",y=", let y1: i64, "..", let y2: i64, ",z=", let z1: i64, "..", let z2: i64) => Op::Off(Cuboid {xs: (x1, x2), ys: (y1, y2), zs: (z1, z2)}),
    } {
        ops.push(op)
    }
    assert_ne!(ops.len(), 0);

    let mut ins: Vec<Cuboid> = vec![];
    let mut outs: Vec<Cuboid> = vec![];
    for op in ops {
        match op {
            Op::Off(c) => {
                let new_outs = ins.iter().filter_map(|d| intersect(&c, d));
                let new_ins: Vec<Cuboid> = outs.iter().filter_map(|d| intersect(&c, d)).collect();
                outs.extend(new_outs);
                ins.extend(new_ins.into_iter());
            }
            Op::On(c) => {
                let new_outs = ins.iter().filter_map(|d| intersect(&c, d));
                let mut new_ins: Vec<Cuboid> =
                    outs.iter().filter_map(|d| intersect(&c, d)).collect();
                new_ins.push(c.clone());
                outs.extend(new_outs);
                ins.extend(new_ins.into_iter());
            }
        }
    }

    println!(
        "[Part 1] {}",
        ins.iter().map(|c| count_points(c)).sum::<i64>()
            - outs.iter().map(|c| count_points(c)).sum::<i64>()
    );
}
