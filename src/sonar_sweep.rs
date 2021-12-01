extern crate itertools;

use std::io::{self, BufRead, BufReader};

fn count_increasing(it: &mut dyn Iterator<Item = i32>) -> i32 {
    let mut prev: i32 = it.next().unwrap();
    let mut result = 0;
    for cur in it {
        if cur > prev {
            result += 1;
        }
        prev = cur;
    }
    result
}

fn main() {
    let depths: Vec<i32> = BufReader::new(io::stdin())
        .lines()
        .filter_map(|value| value.unwrap().parse::<i32>().ok())
        .collect();

    {
        let mut depths_iter  = depths.windows(1).map(|x| x.iter().sum::<i32>());
        println!("[Part 1] Result: {}", count_increasing(&mut depths_iter));
    }
    {
        let mut depths_iter  = depths.windows(3).map(|x| x.iter().sum::<i32>());
        println!("[Part 2] Result: {}", count_increasing(&mut depths_iter));
    }
}
