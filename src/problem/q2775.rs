use std::fmt::Write;
use std::io::{stdin, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    let test_case = iter.next().unwrap();
    let mut v = vec![vec![None; 15]; 15];

    for i in 0..15 {
        v[0][i] = Some(i);
        v[i][1] = Some(1);
    }

    let mut out = String::new();
    for _ in 0..test_case {
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        let answer = match v[a][b] {
            Some(x) => x,
            None => {
                for i in 1..=a {
                    for j in 1..=b {
                        if v[i][j] != None {
                            continue;
                        } else {
                            v[i][j] = Some(v[i - 1][j].unwrap() + v[i][j - 1].unwrap());
                        }
                    }
                }
                v[a][b].unwrap()
            }
        };
        writeln!(out, "{}", answer).unwrap();
    }

    print!("{}", out);
}