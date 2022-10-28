use std::io::{stdin, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let (n, m): (usize, _) = {
        let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };
    buf.clear();
    stdin().read_to_string(&mut buf).unwrap();
    let mut v: Vec<Vec<usize>> = vec![vec![0; m]; n];
    {
        let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
        for row in v.iter_mut() {
            for value in row.iter_mut() {
                *value = iter.next().unwrap();
            }
        }
    }

    let mut max = 0;
    for i in 0..n {
        for j in 0..m {
            if i < n - 3 {
                max = max.max(v[i][j] + v[i + 1][j] + v[i + 2][j] + v[i + 3][j]);
            }
            if j < m - 3 {
                max = max.max(v[i][j] + v[i][j + 1] + v[i][j + 2] + v[i][j + 3]);
            }
            if i < n - 1 && j < m - 1 {
                max = max.max(v[i][j] + v[i + 1][j] + v[i][j + 1] + v[i + 1][j + 1]);
            }
            if i < n - 1 && j < m - 2 {
                max = max.max(v[i][j] + v[i + 1][j] + v[i][j + 1] + v[i][j + 2]);
                max = max.max(v[i][j] + v[i + 1][j] + v[i + 1][j + 1] + v[i + 1][j + 2]);
                max = max.max(v[i][j] + v[i][j + 1] + v[i][j + 2] + v[i + 1][j + 2]);
                max = max.max(v[i + 1][j] + v[i + 1][j + 1] + v[i + 1][j + 2] + v[i][j + 2]);

                max = max.max(v[i][j] + v[i][j + 1] + v[i + 1][j + 1] + v[i + 1][j + 2]);
                max = max.max(v[i + 1][j] + v[i + 1][j + 1] + v[i][j + 1] + v[i][j + 2]);

                max = max.max(v[i][j] + v[i][j + 1] + v[i][j + 2] + v[i + 1][j + 1]);
                max = max.max(v[i + 1][j] + v[i + 1][j + 1] + v[i + 1][j + 2] + v[i][j + 1]);
            }
            if i < n - 2 && j < m - 1 {
                max = max.max(v[i][j] + v[i + 1][j] + v[i + 2][j] + v[i][j + 1]);
                max = max.max(v[i][j] + v[i][j + 1] + v[i + 1][j + 1] + v[i + 2][j + 1]);
                max = max.max(v[i][j] + v[i + 1][j] + v[i + 2][j] + v[i + 2][j + 1]);
                max = max.max(v[i][j + 1] + v[i + 1][j + 1] + v[i + 2][j + 1] + v[i + 2][j]);

                max = max.max(v[i][j] + v[i + 1][j] + v[i + 1][j + 1] + v[i + 2][j + 1]);
                max = max.max(v[i][j + 1] + v[i + 1][j + 1] + v[i + 1][j] + v[i + 2][j]);

                max = max.max(v[i][j] + v[i + 1][j] + v[i + 2][j] + v[i + 1][j + 1]);
                max = max.max(v[i][j + 1] + v[i + 1][j + 1] + v[i + 2][j + 1] + v[i + 1][j]);
            }
        }
    }

    println!("{}", max);
}
