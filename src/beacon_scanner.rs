#[macro_use]
extern crate scan_rules;

use scan_rules::scanner::{Everything, Line, Newline};
use std::collections::{HashMap, HashSet};

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

fn do_overlap(
    scanner: &HashSet<Point>,
    other: &HashSet<Point>,
) -> Option<Point> {
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

fn dfs(
    graph: &HashMap<usize, Vec<(usize, (Point, usize))>>,
    scanners: &HashMap<usize, HashSet<Point>>,
    visited: &mut HashSet<usize>,
    scanner_id: usize,
) -> HashSet<Point> {
    if visited.contains(&scanner_id) {
        return HashSet::new();
    }
    visited.insert(scanner_id);
    let mut res = scanners.get(&scanner_id).unwrap().clone();

    for (other_id, (Point { x, y, z }, rotation_id)) in graph.get(&scanner_id).unwrap_or(&vec![]) {
        res.extend(dfs(graph, scanners, visited, *other_id).iter().map(|p| {
                translate(
                    &rotations(
                        *rotation_id,
                            p,
                    ),
                    &Point {
                        x: *x,
                        y: *y,
                        z: *z,
                    },
                )
        }));
    }

    res
}

// fn scanners(
//     graph: &HashMap<usize, Vec<(usize, (Point, usize))>>,
//     visited: &mut HashSet<usize>,
//     point: Point,
//     scanner_id: usize,
// ): HashSet<Point> {
//     if visited.contains(point) {
//         return HashSet::new();
//     }

// }

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
    let res = dfs(&graph, &scanners, &mut visited, 1);
    println!(
        "Total points: {}",
        res.len()
    );
    assert_eq!(visited.len(), scanners.len());
}
