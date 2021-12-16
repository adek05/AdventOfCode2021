#[macro_use]
extern crate scan_rules;

use std::collections::HashSet;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}

enum Transform {
    FlipX(i32),
    FlipY(i32),
}

fn flip_on_x(p: Point, x: i32) -> Point {
    if p.x <= x {
        p
    } else {
        Point {
            x: x - (p.x - x),
            y: p.y,
        }
    }
}

fn flip_on_y(p: Point, y: i32) -> Point {
    if p.y <= y {
        p
    } else {
        Point {
            x: p.x,
            y: y - (p.y - y),
        }
    }
}

fn transform(p: Point, t: &Transform) -> Point {
    match *t {
        Transform::FlipX(x) => flip_on_x(p, x),
        Transform::FlipY(y) => flip_on_y(p, y),
    }
}

fn main() {
    let mut points: Vec<Point> = vec![];
    let mut transformations: Vec<Transform> = vec![];

    while let Ok(p) = try_readln! {(let x: i32, ",", let y: i32) => Point {x, y}} {
        points.push(p);
    }

    while let Ok(p) = try_readln! {
            ("fold along x=", let x: i32) => Transform::FlipX(x),
            ("fold along y=", let y: i32) => Transform::FlipY(y),
    } {
        transformations.push(p);
    }

    println!(
        "[Part 1] {}",
        points
            .iter()
            .map(|p| transform(p.clone(), transformations.first().unwrap()))
            .collect::<HashSet<Point>>()
            .len()
    );

    let after_folds: Vec<Point> = transformations.iter().fold(points, |ps, t| {
        ps.iter()
            .map(|p| transform(p.clone(), t))
            .collect::<HashSet<Point>>()
            .into_iter()
            .collect()
    });

    let max_y = (after_folds.iter().map(|p| p.x).max().unwrap() + 1) as usize;
    let max_x = (after_folds.iter().map(|p| p.y).max().unwrap() + 1) as usize;

    let mut output: Vec<String> = vec![];
    for _ in 0..max_x {
        output.push(".".repeat(max_y));
    }
    for Point { x, y } in after_folds.iter() {
        output[*y as usize].replace_range(*x as usize..(*x + 1) as usize, "#");
    }
    for x in output {
        println!("{}", x);
    }
}
