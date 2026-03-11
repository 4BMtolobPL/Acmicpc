use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let lines = buf.lines();

    let v: Vec<(Vec<u8>, i32, i32)> = lines
        .skip(1)
        .map(|line| {
            let mut c = line.split_ascii_whitespace();
            let x: Vec<u8> = c
                .next()
                .unwrap()
                .as_bytes()
                .iter()
                .map(|&b| b - b'0')
                .collect();
            (
                x,
                c.next().unwrap().parse().unwrap(),
                c.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut sum = 0;
    for i in 1..=7 {
        for j in (i + 1)..=8 {
            if j == i {
                continue;
            }

            for k in (j + 1)..=9 {
                if k == i || k == j {
                    continue;
                }

                if check_number(&[i, j, k], &v) {
                    sum += 1
                }
                if check_number(&[i, k, j], &v) {
                    sum += 1
                }
                if check_number(&[j, i, k], &v) {
                    sum += 1
                }
                if check_number(&[j, k, i], &v) {
                    sum += 1
                }
                if check_number(&[k, i, j], &v) {
                    sum += 1
                }
                if check_number(&[k, j, i], &v) {
                    sum += 1
                }
            }
        }
    }

    println!("{sum}");
}

fn check_number(target: &[u8], filters: &Vec<(Vec<u8>, i32, i32)>) -> bool {
    for filter in filters {
        let mut strike = 0;
        let mut ball = 0;

        for (i_index, i) in target.iter().enumerate().take(3) {
            for (j_index, j) in filter.0.iter().enumerate().take(3) {
                if *i == *j {
                    if i_index == j_index {
                        strike += 1;
                    } else {
                        ball += 1;
                    }
                }
            }
        }

        if strike != filter.1 || ball != filter.2 {
            return false;
        }
    }
    true
}
