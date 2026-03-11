use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();

    println!("{}", n_queens(n));
}

fn n_queens(n: usize) -> usize {
    let mut v: Vec<usize> = vec![0; n];

    dfs(n, 0, &mut v)
}

fn dfs(n: usize, row: usize, v: &mut Vec<usize>) -> usize {
    let mut sum = 0;

    // 둘수 있는 위치 확인
    let mut va = vec![true; n];
    for (i, val) in v.iter().enumerate().take(row) {
        va[*val] = false;

        let diff = row.abs_diff(i);
        if *val + diff < n {
            va[*val + diff] = false;
        }
        if *val >= diff {
            va[*val - diff] = false;
        }
    }

    for (col, _) in va.iter().enumerate().filter(|(_, val)| **val) {
        if row == n - 1 {
            sum += 1;
        } else {
            v[row] = col;
            sum += dfs(n, row + 1, v);
        }
    }

    sum
}
