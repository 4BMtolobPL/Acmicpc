use std::fmt::Write;
use std::io;
const SUM_SET: [[usize; 10]; 11] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 2, 1, 0, 0, 0, 0, 0, 0, 0],
    [1, 3, 3, 1, 0, 0, 0, 0, 0, 0],
    [1, 4, 6, 4, 1, 0, 0, 0, 0, 0],
    [1, 5, 10, 10, 5, 1, 0, 0, 0, 0],
    [1, 6, 15, 20, 15, 6, 1, 0, 0, 0],
    [1, 7, 21, 35, 35, 21, 7, 1, 0, 0],
    [1, 8, 28, 56, 70, 56, 28, 8, 1, 0],
    [1, 9, 36, 84, 126, 126, 84, 36, 9, 1],
];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());

    let n: usize = iter.next().unwrap();
    let f = iter.next().unwrap();
    let mut stack = vec![];
    let mut used = vec![false; n + 1];

    dfs(n, f, &mut stack, &mut used);

    let mut out = String::new();
    write!(out, "{}", stack.first().unwrap()).unwrap();
    for i in stack.iter().skip(1) {
        write!(out, " {i}").unwrap();
    }
    println!("{out}");
}

fn dfs(n: usize, f: usize, stack: &mut Vec<usize>, used: &mut Vec<bool>) -> bool {
    if stack.len() == n {
        let mut sum = 0;
        for (i, val) in stack.iter().enumerate() {
            sum += val * SUM_SET[n][i];
        }

        return sum == f;
    }

    for i in 1..=n {
        if !used[i] {
            if stack.len() >= n.div_ceil(2) {
                let l = n - stack.len() - 1;
                if stack[l] > i {
                    continue;
                }
            }

            stack.push(i);
            used[i] = true;
            if dfs(n, f, stack, used) {
                return true;
            }
            stack.pop();
            used[i] = false;
        }
    }

    false
}
