#[macro_use]
extern crate scan_rules;

fn part2_cost(pos: i64, crabs: &[i64]) -> i64 {
    crabs
        .iter()
        .map(|x| (pos - x).abs() * ((pos - x).abs() + 1) / 2)
        .sum()
}

fn main() {
    let mut crabs: Vec<i64> = readln! {([let numbers: i64],+) => numbers};
    crabs.sort();

    let final_position = crabs[(crabs.len() + 1) / 2];
    println!(
        "[Part 1] {}",
        crabs
            .iter()
            .map(|pos| (pos - final_position).abs())
            .sum::<i64>()
    );
    println!(
        "[Part 2] {}",
        (*crabs.iter().min().unwrap()..*crabs.iter().max().unwrap())
            .map(|pos| part2_cost(pos, &crabs))
            .min()
            .unwrap()
    );
}
