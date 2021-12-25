use std::collections::{HashMap};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct PlayerState {
    pub id: i32,
    pub position: i32,
    pub score: i32,
}

fn simulate(p1: PlayerState, p2: PlayerState) -> (u64, u64) {
    let dice_distribution: Vec<(i32, u64)> =
        vec![(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    let mut map: HashMap<(PlayerState, PlayerState), u64> = HashMap::new();
    map.insert((p1, p2), 1);

    let mut p1_win: u64 = 0;
    let mut p2_win: u64 = 0;

    while !map.is_empty() {
        let mut new_map: HashMap<(PlayerState, PlayerState), u64> = HashMap::new();
        for ((p1, p2), u_count) in map {
            for (roll_value, freq) in &dice_distribution {
                let mut new_position = p1.position + roll_value;
                if new_position > 10 {
                    new_position -= 10;
                }

                let new_p1 = PlayerState {
                    id: p1.id,
                    position: new_position,
                    score: p1.score + new_position,
                };

                if new_p1.score >= 21 {
                    if new_p1.id == 1 {
                        p1_win += u_count * freq;
                    } else {
                        p2_win += u_count * freq;
                    }
                } else {
                    new_map
                        .entry((p2, new_p1))
                        .and_modify(|x| *x += u_count * freq)
                        .or_insert(u_count * freq);
                }
            }
        }
        map = new_map;
    }
    (p1_win, p2_win)
}

fn main() {
    println!(
        "{:?}",
        simulate(
            PlayerState {
                id: 1,
                position: 9,
                score: 0,
            },
            PlayerState {
                id: 2,
                position: 10,
                score: 0,
            },
        ),
    );
}
