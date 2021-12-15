#[macro_use]
extern crate scan_rules;

use scan_rules::scanner::Word;
use std::collections::{HashMap, HashSet};

fn is_big_cave(name: &str) -> bool {
    name.chars().next().unwrap().is_uppercase()
}

fn dfs(cave: &str, graph: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>, used_twice: bool) -> u32 {
    if cave == "end" {
        return 1;
    }
    let mut new_used_twice = false;
    if !is_big_cave(cave) && visited.contains(cave) {
        if used_twice || cave == "start" {
            return 0;
        } else {
            new_used_twice = true;
        }
    }
    if !is_big_cave(cave) {
        visited.insert(cave.to_owned());
    }
    let mut sum_paths = 0;
    for n in graph.get(cave).unwrap_or(&vec![]) {
        sum_paths += dfs(&n, graph, visited, new_used_twice || used_twice);
    }

    if !is_big_cave(cave) && !new_used_twice {
        visited.remove(cave);
    }
    sum_paths
}

fn main() {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    while let Ok((start, end)) =
        try_readln! {(let start: Word<String>, " - ", let end: Word<String>) => (start, end)}
    {
        graph
            .entry(start.clone())
            .and_modify(|ends| ends.push(end.clone()))
            .or_insert_with(|| vec![end.clone()]);
        graph
            .entry(end.clone())
            .and_modify(|starts| starts.push(start.clone()))
            .or_insert_with(|| vec![start]);
    }

    let mut visited: HashSet<String> = HashSet::new();
    println!("[Part 1] {}", dfs("start", &graph, &mut visited, true));
    let mut visited: HashSet<String> = HashSet::new();
    println!("[Part 2] {}", dfs("start", &graph, &mut visited, false));
}
