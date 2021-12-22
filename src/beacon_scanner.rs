#[macro_use]
extern crate scan_rules;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

fn rotations(rotation_number: usize, &Point { x, y, z }: &Point) -> Point {
    vec![
        Point { x: x, y: y, z: z },
        Point { x: x, y: z, z: -y },
        Point { x: x, y: -y, z: -z },
        Point { x: x, y: -z, z: y },
        Point { x: -x, y: -y, z: z },
        Point { x: -x, y: z, z: y },
        Point { x: -x, y: y, z: -z },
        Point {
            x: -x,
            y: -z,
            z: -y,
        },
        Point { x: y, y: -x, z: z },
        Point { x: y, y: z, z: x },
        Point { x: y, y: x, z: -z },
        Point { x: y, y: -z, z: -x },
        Point { x: -y, y: x, z: z },
        Point { x: -y, y: z, z: -x },
        Point {
            x: -y,
            y: -x,
            z: -z,
        },
        Point { x: -y, y: -z, z: x },
        Point { x: z, y: x, z: y },
        Point { x: z, y: y, z: -x },
        Point { x: z, y: -x, z: -y },
        Point { x: z, y: -y, z: x },
        Point { x: -z, y: -x, z: y },
        Point { x: -z, y: y, z: x },
        Point { x: -z, y: x, z: -y },
        Point {
            x: -z,
            y: -y,
            z: -x,
        },
    ][rotation_number]
        .clone()
}

fn translate(p: &Point, v: &Point) -> Point {
    Point {
        x: p.x + v.x,
        y: p.y + v.y,
        z: p.z + v.z,
    }
}

fn are_same(s1: &HashSet<Point>, s2: &HashSet<Point>, at_least_n: usize) -> bool {
    let intersection_size = s1.intersection(s2).count();
    if intersection_size > 10 {
        println!("Intersection_size {}", intersection_size);
    }
    intersection_size >= at_least_n
}

fn do_overlap(scanner: &HashSet<Point>, other: &HashSet<Point>) -> Option<Point> {
    for point in scanner {
        for rotated_point in other {
            let dv = Point {
                x: point.x - rotated_point.x,
                y: point.y - rotated_point.y,
                z: point.z - rotated_point.z,
            };
            if are_same(
                scanner,
                &other.iter().map(|p| translate(p, &dv)).collect(),
                12,
            ) {
                return Some(dv);
            }
        }
    }
    None
}

fn normalize_coords(rotation_id: usize, translation_v: &Point, p: &Point) -> Point {
    translate(&rotations(rotation_id, p), translation_v)
}

fn dfs(
    graph: &HashMap<usize, Vec<(usize, (Point, usize))>>,
    scanners: &HashMap<usize, HashSet<Point>>,
    visited: &mut HashSet<usize>,
    scanner_id: usize,
) -> (HashSet<Point>, HashSet<Point>) {
    if visited.contains(&scanner_id) {
        return (HashSet::new(), HashSet::new());
    }
    visited.insert(scanner_id);
    let mut beacons = scanners.get(&scanner_id).unwrap().clone();
    let mut scanner_coords = HashSet::from_iter(vec![Point{x: 0, y: 0, z: 0}]);

    for (other_id, (translation_v, rotation_id)) in graph.get(&scanner_id).unwrap_or(&vec![]) {
        let (bs, scans) = dfs(graph, scanners, visited, *other_id);
        beacons.extend(
            bs
                .iter()
                .map(|p| normalize_coords(*rotation_id, translation_v, p)),
        );
        scanner_coords.extend(scans.iter()
                .map(|p| normalize_coords(*rotation_id, translation_v, p)),
        );
    }

    (beacons, scanner_coords)
}

fn main() {
    let mut scanners: HashMap<usize, HashSet<Point>> = HashMap::new();

    while let Ok(scanner_id) =
        try_readln! { ("--- scanner ", let scanner_id: usize, "---") => scanner_id }
    {
        let mut points: HashSet<Point> = HashSet::new();
        while let Ok(p) =
            try_readln! { (let x: i32, ",", let y: i32, ",", let z: i32) => Point {x, y, z}}
        {
            points.insert(p);
        }
        scanners.insert(scanner_id, points);
    }

    let mut visited: HashSet<usize> = HashSet::new();
    let mut graph: HashMap<usize, Vec<(usize, (Point, usize))>> = HashMap::new();

    for (i, scanner) in &scanners {
        for (j, other) in &scanners {
            if i == j {
                continue;
            }
            for rotation_id in 0..24 {
                let rotated: HashSet<Point> =
                    other.iter().map(|p| rotations(rotation_id, p)).collect();
                if let Some(dv) = do_overlap(scanner, &rotated) {
                    graph
                        .entry(*i)
                        .and_modify(|x| x.push((*j, (dv.clone(), rotation_id))))
                        .or_insert(vec![(*j, (dv.clone(), rotation_id))]);
                    println!(
                        "Scanners {} and {} overlap with offset: {:?} and rotation_id {}",
                        i, j, dv, rotation_id
                    );
                }
            }
        }
    }
    let (beacons, scanners_coords) = dfs(&graph, &scanners, &mut visited, 1);
    assert_eq!(visited.len(), scanners.len());
    assert_eq!(scanners_coords.len(), scanners.len());

    println!("[Part 1] {}", beacons.len());
    let max_dist = scanners_coords.iter().cloned().flat_map(|p| scanners_coords.iter().cloned().map(move |r| (p.x - r.x).abs() + (p.y - r.y).abs() + (p.z - r.z).abs())).max().unwrap();
    println!("[Part 2] {}", max_dist);
}
