#[macro_use]
extern crate scan_rules;

use scan_rules::scanner::Word;
use std::collections::HashMap;

fn sum_frequencies(h1: &HashMap<char, u64>, h2: &HashMap<char, u64>) -> HashMap<char, u64> {
    let mut h = h1.clone();
    h2.iter().for_each(|c| {
        h.entry(*c.0).and_modify(|freq| *freq += *c.1).or_insert(*c.1);
    });
    h
}

fn count(
    rules: &HashMap<String, char>,
    memo: &mut HashMap<(String, u64), HashMap<char, u64>>,
    depth: u64,
    s: String,
) -> HashMap<char, u64> {
    if let Some(res) = memo.get(&(s.clone(), depth)) {
        return res.clone();
    }
    let char1: char = s.chars().nth(0).unwrap();
    let char2: char = s.chars().nth(1).unwrap();

    let mut result: HashMap<char, u64> = HashMap::new();
    result.entry(char1).and_modify(|x| *x += 1).or_insert(1);
    result.entry(char2).and_modify(|x| *x += 1).or_insert(1);

    if depth == 0 {
        result
    } else if let Some(insertion) = rules.get(&format!("{}{}", char1, char2)) {
        let mut res = sum_frequencies(
            &count(&rules, memo, depth - 1, format!("{}{}", char1, insertion)),
            &count(&rules, memo, depth - 1, format!("{}{}", insertion, char2)),
        );
        res.entry(*insertion).and_modify(|x| *x -= 1);
        memo.insert((s, depth), res.clone());
        res
    } else {
        HashMap::new()
    }
}

fn main() {
    let mut pattern = readln! {(let p: Word<String>) => p};
    readln! {("\n") => ()};

    let mut rules: HashMap<String, char> = HashMap::new();
    while let Ok((from, to)) =
        try_readln! { (let from: Word<String>, " -> ", let to: char) => (from, to)}
    {
        rules.insert(from.clone(), to);
    }

    // for _ in 0..10 {
    //     let mut new_pattern: String = "".to_string();
    //     let mut iter = pattern.chars();
    //     let mut first: char = iter.next().unwrap();
    //     while let Some(second) = iter.next() {
    //         new_pattern.push(first);
    //         if let Some(insertion) = rules.get(&format!("{}{}", first, second)) {
    //             new_pattern.push(*insertion);
    //         }
    //         first = second;
    //     }
    //     new_pattern.push(first);
    //     pattern = new_pattern;
    // }
    // let mut frequency: HashMap<char, usize> = HashMap::new();
    // for c in pattern.chars() {
    //     frequency.entry(c).and_modify(|x| *x += 1).or_insert(1);
    // }

    // println!(
    //     "[Part 1] {}",
    //     frequency.iter().max_by_key(|x| x.1).unwrap().1
    //         - frequency.iter().min_by_key(|x| x.1).unwrap().1
    // );

    let mut memo: HashMap<(String, u64), HashMap<char, u64>> = HashMap::new();
    let mut iter = pattern.chars();
    let mut first: char = iter.next().unwrap();
    let mut frequencies: HashMap<char, u64> = HashMap::new();
    while let Some(second) = iter.next() {
        frequencies = sum_frequencies(
            &count(&rules, &mut memo, 40, format!("{}{}", first, second)),
            &frequencies,
        );
        frequencies.entry(second).and_modify(|x| *x -= 1);
        first = second;
    }
    frequencies.entry(first).and_modify(|x| *x += 1);

    // let p1 = count(&rules, &mut memo, 50, "NN".to_owned());
    // println!("{:?}", p1);
    println!(
        "[Part 1] {}",
        frequencies.iter().max_by_key(|x| x.1).unwrap().1
            - frequencies.iter().min_by_key(|x| x.1).unwrap().1
    );
}
