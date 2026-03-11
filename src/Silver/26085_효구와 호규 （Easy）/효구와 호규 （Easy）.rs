use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    let (n, m): (usize, usize) = {
        let mut iter = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    if (n * m) % 2 != 0 {
        println!("-1");
    } else {
        let v: Vec<Vec<i32>> = lines
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect()
            })
            .collect();

        if v.iter()
            .flat_map(|row| row.iter().filter(|x| **x == 0))
            .count()
            % 2
            == 1
        {
            println!("-1");
        } else if check_connect(&v, n, m) {
            println!("1");
        } else {
            println!("-1");
        }
    }
}

fn check_connect(v: &[Vec<i32>], row_len: usize, col_len: usize) -> bool {
    for i in 0..(row_len - 1) {
        for j in 0..(col_len - 1) {
            if v[i][j] == v[i][j + 1] || v[i][j] == v[i + 1][j] {
                return true;
            }
        }
    }

    if v[row_len - 2][col_len - 1] == v[row_len - 1][col_len - 1]
        || v[row_len - 1][col_len - 2] == v[row_len - 1][col_len - 1]
    {
        return true;
    }

    false
}
