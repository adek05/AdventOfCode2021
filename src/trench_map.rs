#[macro_use]
extern crate scan_rules;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use scan_rules::scanner::{Everything, NonSpace};

#[derive(PartialEq, Hash, Eq)]
struct Point {
    pub x: i32,
    pub y: i32,
}

struct Image {
    pub subset: HashMap<Point, bool>,
    pub infinity: bool,
}

fn get_neighbors(p: &Point) -> Vec<Point> {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .map(|(dx, dy)| Point {
        x: p.x + dx,
        y: p.y + dy,
    })
    .collect()
}

fn get_value(p: &Point, image: &Image) -> bool {
    *image.subset.get(p).unwrap_or(&image.infinity)
}

fn enhance(image: &Image, algorithm: &[bool]) -> Image {
    let ps: HashSet<Point> =
        HashSet::from_iter(image.subset.iter().map(|(p, _)| get_neighbors(p)).flatten());
    let mut new_subset: HashMap<Point, bool> = HashMap::new();
    for p in ps {
        let enhanced_value: bool = algorithm[get_neighbors(&p)
            .into_iter()
            .map(|n| get_value(&n, image) as usize)
            .fold(0, |acc, x| 2 * acc + x)];
        new_subset.insert(p, enhanced_value);
    }

    Image {
        subset: new_subset,
        infinity: if image.infinity {
            algorithm[511]
        } else {
            algorithm[0]
        },
    }
}

fn main() {
    let line = readln! { (let s: NonSpace) => s.to_string()};
    let algorithm: Vec<bool> = line.chars().map(|c| c != '.').collect();
    let mut x = 0;
    let mut subset: HashMap<Point, bool> = HashMap::new();
    readln! {(let x: Everything) => ()};

    while let Ok(line) = try_readln! { (let s: NonSpace) => s.to_string()} {
        line.chars().enumerate().for_each(|(y, c)| {
            subset.insert(Point { x, y: y as i32 }, c != '.');
        });
        x += 1;
    }

    let image = Image {
        subset,
        infinity: false,
    };

    {
        let enhanced_image = (0..50).fold(image, |acc, _| enhance(&acc, &algorithm));
        let res: usize = enhanced_image
            .subset
            .into_iter()
            .map(|(_, v)| v as usize)
            .sum();
        println!("[Part 2]: {}", res);
    }
}
