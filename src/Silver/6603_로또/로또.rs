use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let lines = buf.lines();

    let mut out = String::new();
    for line in lines {
        if line == "0" {
            break;
        }

        let v: Vec<i32> = line
            .split_ascii_whitespace()
            .skip(1)
            .map(|x| x.parse().unwrap())
            .collect();

        let mut stack = Vec::new();
        dfs(0, &mut stack, &v, &mut out);
        writeln!(out).unwrap();
    }

    print!("{}", out.trim_end());
}

fn dfs(index: usize, stack: &mut Vec<i32>, v: &[i32], out: &mut String) {
    if stack.len() == 6 {
        let mut iter = stack.iter();
        let first = iter.next().unwrap();

        write!(out, "{first}").unwrap();

        for i in iter {
            write!(out, " {i}").unwrap();
        }
        writeln!(out).unwrap();
    } else {
        for (i, val) in v.iter().enumerate().skip(index) {
            stack.push(*val);
            dfs(i + 1, stack, v, out);
            stack.pop();
        }
    }
}
