#[macro_use]
extern crate scan_rules;

use scan_rules::scanner::Word;
use std::fmt::Binary;

#[derive(Debug)]
struct Literal {
    pub version: i32,
    pub number: i64,
}

#[derive(Debug)]
struct Operator {
    pub version: i32,
    pub op_type: i32,
    pub packets: Vec<Packet>,
}

#[derive(Debug)]
enum Packet {
    Literal(Literal),
    Operator(Operator),
}

fn read_bits(n: usize, iter: &mut dyn Iterator<Item = char>) -> String {
    let mut res: String = String::new();
    for _ in 0..n {
        res.push(iter.next().unwrap());
    }
    res
}

fn read_literal(version: i32, packet_type: i32, iter: &mut dyn Iterator<Item = char>) -> Packet {
    assert_eq!(packet_type, 4, "Only literals here");
    let mut control;
    let mut number: String = String::new();
    loop {
        control = iter.next().unwrap();
        number.push_str(&read_bits(4, iter));
        if control == '0' {
            break;
        }
    }

    Packet::Literal(Literal {
        version,
        number: i64::from_str_radix(&number, 2).unwrap(),
    })
}

fn read_packet(iter: &mut dyn Iterator<Item = char>) -> Packet {
    let version: i32 = i32::from_str_radix(&read_bits(3, iter), 2).unwrap();
    let packet_type: i32 = i32::from_str_radix(&read_bits(3, iter), 2).unwrap();

    if packet_type == 4 {
        return read_literal(version, packet_type, iter);
    }
    let length_type = iter.next().unwrap();
    match length_type {
        '0' => {
            let len_to_read = usize::from_str_radix(&read_bits(15, iter), 2).unwrap();
            let bits = read_bits(len_to_read, iter);
            let mut inner_iter = bits.chars().peekable();
            let mut inner_packets: Vec<Packet> = Vec::new();
            while inner_iter.peek().is_some() {
                inner_packets.push(read_packet(&mut inner_iter));
            }
            return Packet::Operator(Operator {
                version,
                op_type: packet_type,
                packets: inner_packets,
            });
        }
        '1' => {
            let packets_to_read = usize::from_str_radix(&read_bits(11, iter), 2).unwrap();
            let mut inner_packets: Vec<Packet> = Vec::new();
            for _ in 0..packets_to_read {
                inner_packets.push(read_packet(iter));
            }
            return Packet::Operator(Operator {
                version,
                op_type: packet_type,
                packets: inner_packets,
            });
        }
        x => panic!("Invalid lenght_type. Expected 0 or 1. Got {}", x),
    }

    panic!()
}

fn sum_versions(p: &Packet) -> i32 {
    match p {
        Packet::Literal(l) => l.version,
        Packet::Operator(o) => o.packets.iter().map(|p| sum_versions(p)).sum::<i32>() + o.version,
    }
}

fn eval(p: &Packet) -> i64 {
    match p {
        Packet::Literal(l) => l.number as i64,
        Packet::Operator(o) => match o.op_type {
            0 => o.packets.iter().map(|p| eval(p)).sum(),
            1 => o.packets.iter().map(|p| eval(p)).product(),
            2 => o.packets.iter().map(|p| eval(p)).min().unwrap(),
            3 => o.packets.iter().map(|p| eval(p)).max().unwrap(),
            5 => (eval(&o.packets[0]) > eval(&o.packets[1]) )as i64,
            6 => (eval(&o.packets[0]) < eval(&o.packets[1]) )as i64,
            7 => (eval(&o.packets[0]) == eval(&o.packets[1])) as i64,
            _ => panic!("Unexpected op type"),
        },
    }
}

fn main() {
    let transmission = readln! {(let p: Word<String>) => p};
    let binary_transmission: String = transmission
        .chars()
        .map(|c| format!("{:04b}", i32::from_str_radix(&c.to_string(), 16).unwrap()))
        .collect::<Vec<String>>()
        .join("");
    let packet = read_packet(&mut binary_transmission.chars());
    println!("[Part 1] {}", sum_versions(&packet));
    println!("[Part 2] {}", eval(&packet));
}
