extern crate itertools;

use std::io::{self, BufRead, BufReader};

struct Frequency {
    ones: i32,
    zeros: i32,
}

fn histogram(v: &[String], pos: usize) -> Frequency {
    let mut f = Frequency { ones: 0, zeros: 0 };
    for number in v.iter() {
        match number.chars().nth(pos) {
            Some('0') => {
                f = Frequency {
                    ones: f.ones,
                    zeros: f.zeros + 1,
                }
            }
            Some('1') => {
                f = Frequency {
                    ones: f.ones + 1,
                    zeros: f.zeros,
                }
            }
            _ => panic!("Unexpected numbers"),
        }
    }
    f
}

fn filter_value(v: Vec<String>, pos: usize, value: char) -> Vec<String> {
    v.into_iter()
        .filter(|n| n.chars().nth(pos) == Some(value))
        .collect()
}

fn main() {
    let numbers: Vec<String> = BufReader::new(io::stdin())
        .lines()
        .filter_map(|value| value.ok())
        .collect();

    let mut oxygen_generator = numbers.clone();
    let mut co2_scrubbing = numbers.clone();

    let mut gamme_rate = "".to_owned();
    let mut epsilon_rate = "".to_owned();

    for i in 0..numbers.first().unwrap().len() {
        let f = histogram(&numbers, i);
        if f.ones >= f.zeros {
            gamme_rate.push_str("1");
            epsilon_rate.push_str("0");
        } else {
            gamme_rate.push_str("0");
            epsilon_rate.push_str("1");
        }
        if oxygen_generator.len() > 1 {
            let oxygen_generator_histogram = histogram(&oxygen_generator, i);
            if oxygen_generator_histogram.ones >= oxygen_generator_histogram.zeros {
                oxygen_generator = filter_value(oxygen_generator, i, '1');
            } else {
                oxygen_generator = filter_value(oxygen_generator, i, '0');
            }
        }
        if co2_scrubbing.len() > 1 {
            let co2_scrubbing_histogram = histogram(&co2_scrubbing, i);
            if co2_scrubbing_histogram.ones >= co2_scrubbing_histogram.zeros {
                co2_scrubbing = filter_value(co2_scrubbing, i, '0');
            } else {
                co2_scrubbing = filter_value(co2_scrubbing, i, '1');
            }
        }
    }

    let gamma = isize::from_str_radix(&gamme_rate, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon_rate, 2).unwrap();
    println!("[Part 1] {}", gamma * epsilon);

    let life = isize::from_str_radix(&co2_scrubbing.first().unwrap(), 2).unwrap();
    let oxygen = isize::from_str_radix(&oxygen_generator.first().unwrap(), 2).unwrap();
    println!("[Part 2] {}", life * oxygen);
}
