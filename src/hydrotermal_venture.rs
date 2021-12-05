#[macro_use]
extern crate scan_rules;
extern crate itertools;

use std::collections::HashMap;

use std::io::{self, BufRead, BufReader};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug)]
struct Line {
    pub begin: Point,
    pub end: Point,
}

impl Line {
    fn create(p: Point, q: Point) -> Self {
        if p.x < q.x {
            Self { begin: p, end: q }
        } else if p.x == q.x {
            if p.y <= q.y {
                Self { begin: p, end: q }
            } else {
                Self { begin: q, end: p }
            }
        } else {
            Self { begin: q, end: p }
        }
    }

    fn enumerate_points(&self) -> Vec<Point> {
        if self.is_horizontal() {
            (self.begin.x..self.end.x + 1)
                .map(|x| Point { x, y: self.begin.y })
                .collect()
        } else if self.is_vertical() {
            (self.begin.y..self.end.y + 1)
                .map(|y| Point { x: self.begin.x, y })
                .collect()
        } else {
            let mut res = vec![];
            if self.begin.y <= self.end.y {
                for i in 0..self.end.y-self.begin.y+1 {
                    res.push(Point{x: self.begin.x+i, y: self.begin.y+i});
                }
            } else {
                for i in 0..self.begin.y-self.end.y+1 {
                    res.push(Point{x: self.begin.x+i, y: self.begin.y-i});
                }
            }
            res
        }
    }

    fn is_horizontal(&self) -> bool {
        self.begin.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.begin.x == self.end.x
    }

    fn is_horizontal_or_vertical(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }
}

fn main() {
    let lines: Vec<Line> = BufReader::new(io::stdin()).lines().map(|line| line.unwrap()).map(
        |line| scan!(&line; (let x1: i32, ",", let y1: i32, "->", let x2: i32, ",", let y2:i32) =>  Line::create(Point{x: x1, y: y1}, Point{x:x2, y: y2})).unwrap()
    ).collect();

    {
        let h_or_v_lines: Vec<Line> = lines
            .iter()
            .filter(|l| l.is_horizontal_or_vertical())
            .cloned()
            .collect();
        let mut vents: HashMap<Point, usize> = HashMap::new();
        for line in h_or_v_lines {
            for point in line.enumerate_points() {
                vents.entry(point).and_modify(|x| *x += 1).or_insert(1);
            }
        }
        println!("[Part 1] {}", vents.into_iter().filter(|(_, cnt)| *cnt > 1).count());
    }
    {
        let mut vents: HashMap<Point, usize> = HashMap::new();
        for line in lines {
            for point in line.enumerate_points() {
                vents.entry(point).and_modify(|x| *x += 1).or_insert(1);
            }
        }
        println!("[Part 2] {}", vents.into_iter().filter(|(_, cnt)| *cnt > 1).count());
    }
}
