#[macro_use]
extern crate scan_rules;

use std::collections::HashSet;

struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Velocity {
    pub x: i32,
    pub y: i32,
}

fn is_on_platform(platform_xs: (i32, i32), platform_ys: (i32, i32), p: &Point) -> bool {
    assert_eq!(platform_xs.0 < platform_xs.1, true);
    assert_eq!(platform_ys.0 < platform_ys.1, true);

    platform_xs.0 <= p.x && p.x <= platform_xs.1 && platform_ys.0 <= p.y && p.y <= platform_ys.1
}

fn will_hit(platform_xs: (i32, i32), platform_ys: (i32, i32), velocity: &Velocity) -> bool {
    assert_eq!(platform_xs.0 <= platform_xs.1, true);
    assert_eq!(platform_ys.0 <= platform_ys.1, true);

    let mut probe_position = Point { x: 0, y: 0 };
    let mut cur_velocity = velocity.clone();
    while probe_position.y >= platform_ys.0 {
        probe_position = Point {
            x: probe_position.x + cur_velocity.x,
            y: probe_position.y + cur_velocity.y,
        };
        if is_on_platform(platform_xs, platform_ys, &probe_position) {
            return true;
        }

        cur_velocity = Velocity {
            x: if cur_velocity.x == 0 {
                0
            } else {
                cur_velocity.x - 1
            },
            y: cur_velocity.y - 1,
        }
    }

    false
}

fn main() {
    // target area: x=277..318, y=-92..-53
    let (xs, ys) = readln! { ("target area: x=", let x0: i32, "..", let x1: i32, ", y=", let y0: i32, "..", let y1: i32) => ((x0, x1), (y0, y1))};
    let velocity_xs = 0..(xs.1 + 2);
    let velocity_ys = (ys.0-2)..(-ys.0 + 2);
    let mut results: HashSet<Velocity> = HashSet::new();
    for v_x in velocity_xs {
        for v_y in velocity_ys.clone() {
            let v = Velocity { x: v_x, y: v_y };
            if will_hit(xs, ys, &v) {
                results.insert(v);
            }
        }
    }
    println!("[Part 2] {}", results.len());
}
