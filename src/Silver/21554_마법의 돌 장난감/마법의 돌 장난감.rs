use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut toy_list: Vec<usize> = buf
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut sum = 0;
    let mut out = String::new();
    let mut lhs = 0;

    while lhs < toy_list.len() {
        if toy_list[lhs] == lhs + 1 {
            lhs += 1;
            continue;
        }

        let position = toy_list.iter().position(|&x| x == lhs + 1).unwrap();
        let mut rhs = position;
        while rhs < toy_list.len() - 1 && toy_list[rhs + 1] == toy_list[rhs] + 1 {
            rhs += 1;
        }

        if let Some(s) = toy_list.get_mut(lhs..=rhs) {
            s.reverse();
            sum += 1;

            if sum > 100 {
                break;
            }
            writeln!(out, "{} {}", lhs + 1, rhs + 1).unwrap();
        }
    }

    if sum > 100 {
        println!("-1");
    } else {
        print!("{}\n{}", sum, out);
    }
}
