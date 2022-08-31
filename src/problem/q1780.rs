use std::io::{stdin, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let lines = buf.lines();
    let mut v: Vec<Vec<i8>> = Vec::new();

    for line in lines.skip(1) {
        v.push(
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
        );
    }

    let (a, b, c) = paper(&v, (0, 0), v.len());
    println!("{}\n{}\n{}", a, b, c);
}

fn paper(page: &[Vec<i8>], left_top: (usize, usize), n: usize) -> (i32, i32, i32) {
    let (x, y) = left_top;
    if is_clean(page, left_top, n) {
        match page[x][y] {
            -1 => (1, 0, 0),
            0 => (0, 1, 0),
            1 => (0, 0, 1),
            _ => (0, 0, 0),
        }
    } else {
        let mut sum = (0, 0, 0);
        for line in (x..(x + n)).step_by(n / 3) {
            for i in (y..(y + n)).step_by(n / 3) {
                let r = paper(page, (line, i), n / 3);
                sum.0 += r.0;
                sum.1 += r.1;
                sum.2 += r.2;
            }
        }
        sum
    }
}

fn is_clean(page: &[Vec<i8>], left_top: (usize, usize), n: usize) -> bool {
    let (x, y) = left_top;
    let standard = page[x][y];
    let mut r = true;
    for line in page.iter().skip(x).take(n) {
        for item in line.iter().skip(y).take(n) {
            if standard != *item {
                r = false;
            }
        }
    }
    r
}
