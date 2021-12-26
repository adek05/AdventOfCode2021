use std::io::{self, BufRead, BufReader};

// fn get_value(map: &Vec<String> x: usize, y: usize) -> char {
//     map[x ]

// }

fn main() {
    let mut map: Vec<Vec<char>> = BufReader::new(io::stdin())
        .lines()
        .filter_map(|value| value.map(|x| x.chars().collect()).ok())
        .collect();
    let xs: usize = map.len();
    let ys: usize = map[0].len();

    let mut step_count: usize = 0;
    loop {
        let mut new_map: Vec<Vec<char>> = (0..xs).map(|_| (0..ys).map(|_| '.').collect()).collect();
        step_count += 1;

        // println!("=====");
        // for xs in &map {
        //     println!("{}", xs.iter().collect::<String>());
        // }

        for x in 0..xs {
            for y in  0..ys {
                if map[x][y] == '>' && map[x][(y+1)%ys] == '.' {
                    new_map[x][(y+1)%ys] = '>'
                } else if map[x][y] == '>' {
                    new_map[x][y]  = '>';
                }
            }
        }

        for x in 0..xs {
            for y in  0..ys {
                if map[x][y] == 'v' && new_map[(x+1)%xs][y] == '.' && map[(x+1)%xs][y] != 'v' {
                    new_map[(x+1)%xs][y] = 'v';
                } else if map[x][y] == 'v' {
                    new_map[x][y] = 'v';
                }
            }
        }

        if map == new_map {
            break;
        }
        map = new_map;
    }
    println!("[Part 1] {}", step_count);
}
