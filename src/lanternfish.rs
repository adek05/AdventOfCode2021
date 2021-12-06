#[macro_use]
extern crate scan_rules;

fn count_at_day(fish: &[usize], day: usize) -> i64 {
    let mut fish_count: i64 = fish.len() as i64;
    let mut new_fish_day: Vec<i64> = vec![0; 280];
    for age in fish {
        new_fish_day[*age] += 1;
    }

    for i in 0..day {
        fish_count += new_fish_day[i];
        new_fish_day[i + 7] += new_fish_day[i];
        new_fish_day[i + 9] += new_fish_day[i];
    }
    fish_count
}

fn main() {
    let fish: Vec<usize> = readln! {([let numbers: usize],+) => numbers};

    println!("[Part 1] {}", count_at_day(&fish, 80));
    println!("[Part 2] {}", count_at_day(&fish, 256));
}
