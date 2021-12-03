#[macro_use]
extern crate scan_rules;
extern crate itertools;

use std::io::{self, BufRead, BufReader};

enum Command {
    Up(i64),
    Down(i64),
    Forward(i64),
}

struct Position {
    pub x: i64,
    pub y: i64,
    pub aim: i64,
}

fn to_command(c: String) -> Result<Command, scan_rules::ScanError> {
    scan! { &c;
        ("forward", let f: i64) => Command::Forward(f),
        ("down", let d: i64) => Command::Down(d),
        ("up", let u: i64) => Command::Up(u),
    }
}

fn run_command_1(pos: Position, command: &Command) -> Position {
    match command {
        Command::Up(u) => Position {x: pos.x, y: pos.y - u, aim: pos.aim},
        Command::Down(d) => Position {x: pos.x, y: pos.y + d, aim: pos.aim},
        Command::Forward(f) => Position {x: pos.x + f, y: pos.y, aim: pos.aim}
    }
}

fn run_command_2(pos: Position, command: &Command) -> Position {
    match command {
        Command::Up(u) => Position {x: pos.x, y: pos.y, aim: pos.aim - u},
        Command::Down(d) => Position {x: pos.x, y: pos.y, aim: pos.aim + d},
        Command::Forward(f) => Position {x: pos.x + f, y: pos.y + pos.aim * f, aim: pos.aim}
    }
}

fn main() {
    let moves: Vec<Command> = BufReader::new(io::stdin())
        .lines()
        .map(|value| to_command(value.unwrap()).unwrap())
        .collect();
    
        {}
    let final_position_1 = moves.iter().fold(Position {x: 0, y: 0, aim: 0}, run_command_1);
    println!("[Part 1] {}", final_position_1.x * final_position_1.y);

    let final_position_2 = moves.iter().fold(Position {x: 0, y: 0, aim: 0}, run_command_2);
    println!("[Part 2] {}", final_position_2.x * final_position_2.y);
}