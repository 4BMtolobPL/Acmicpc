use std::{
    collections::VecDeque,
    io::{stdin, stdout, BufWriter, Read, Write},
};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let test_case: usize = lines.next().unwrap().parse().unwrap();

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);

    for _ in 0..test_case {
        let p = lines.next().unwrap();
        lines.next();
        let pattern: &[_] = &['[', ']'];
        let mut arr: VecDeque<&str> = lines
            .next()
            .unwrap()
            .trim_matches(pattern)
            .split(',')
            .filter(|x| !x.is_empty())
            .collect();

        let mut is_forward = true;
        let mut is_error = false;
        for c in p.chars() {
            match c {
                'R' => is_forward = !is_forward,
                'D' => {
                    if arr.is_empty() {
                        is_error = true;
                        break;
                    }
                    if is_forward {
                        arr.pop_front();
                    } else {
                        arr.pop_back();
                    }
                }
                _ => break,
            }
        }
        if is_error {
            writeln!(out, "error").unwrap();
        } else {
            writeln!(
                out,
                "[{}]",
                if is_forward {
                    Vec::from(arr).join(",")
                } else {
                    arr.into_iter().rev().collect::<Vec<&str>>().join(",")
                }
            )
            .unwrap();
        }
    }
}
