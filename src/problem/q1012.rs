use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let test_case = lines.next().unwrap().parse().unwrap();

    let mut out = String::new();
    for _ in 0..test_case {
        let mut iter = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap());
        let width = iter.next().unwrap();
        let height = iter.next().unwrap();
        let cabbage = iter.next().unwrap();

        let mut farm = vec![vec![None; height]; width];
        for _ in 0..cabbage {
            let (x, y) = to_point(lines.next().unwrap());
            farm[x][y] = Some(1);
        }

        let mut count = 0;
        for i in 0..farm.len() {
            for j in 0..farm[i].len() {
                if farm[i][j] != None {
                    count += 1;
                    let mut stack = vec![(i, j)];

                    while !stack.is_empty() {
                        let (x, y) = stack.pop().unwrap();
                        match farm[x][y] {
                            Some(_) => {
                                farm[x][y] = None;
                                if x != (farm.len() - 1) {
                                    stack.push((x + 1, y));
                                }
                                if x != 0 {
                                    stack.push((x - 1, y));
                                }
                                if y != (farm[i].len() - 1) {
                                    stack.push((x, y + 1));
                                }
                                if y != 0 {
                                    stack.push((x, y - 1));
                                }
                            }
                            None => continue,
                        }
                    }
                }
            }
        }
        writeln!(out, "{}", count).unwrap();
    }

    print!("{}", out);
}

fn to_point(line: &str) -> (usize, usize) {
    let mut iter = line.split_whitespace().map(|x| x.parse().unwrap());
    (iter.next().unwrap(), iter.next().unwrap())
}
