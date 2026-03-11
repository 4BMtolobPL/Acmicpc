use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_ascii_whitespace();
    let _ = iter.next().unwrap();

    let mut time = 0;
    let mut score = 0;
    let mut rewards = vec![0; 4];
    let mut points_per_turn = 1;
    let mut time_per_turn = 4;

    for i in iter {
        if time > 240 || i.as_bytes() == b"1" {
            settlement(score, &mut rewards);
            time = 0;
            score = 0;
            points_per_turn = 1;
            time_per_turn = 4;
        }

        match i.as_bytes() {
            b"1" => {
                settlement(score, &mut rewards);
                time = 0;
                score = 0;
                points_per_turn = 1;
                time_per_turn = 4;
                continue;
            }
            b"2" => {
                if points_per_turn > 1 {
                    points_per_turn /= 2;
                } else {
                    time_per_turn += 2;
                }
            }
            b"3" => {}
            b"4" => {
                time += 56;
            }
            b"5" => {
                if time_per_turn > 1 {
                    time_per_turn -= 1;
                }
            }
            b"6" => {
                if points_per_turn < 32 {
                    points_per_turn *= 2;
                }
            }
            _ => unreachable!("주사위는 1~6값이 주어진다."),
        }

        score += points_per_turn;
        time += time_per_turn;
    }

    let mut out = String::with_capacity(8);
    for reward in rewards.iter() {
        writeln!(out, "{reward}").unwrap();
    }

    print!("{out}");
}

fn settlement(score: i32, rewards: &mut [i32]) {
    if (35..65).contains(&score) {
        rewards[0] += 1;
    } else if (65..95).contains(&score) {
        rewards[1] += 1;
    } else if (95..125).contains(&score) {
        rewards[2] += 1;
    } else if score >= 125 {
        rewards[3] += 1;
    }
}
