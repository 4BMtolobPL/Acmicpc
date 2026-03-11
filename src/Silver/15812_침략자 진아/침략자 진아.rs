use std::cmp::min;
use std::io;
use std::io::stdin;

fn main() {
    let buf = io::read_to_string(stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace();

    let (n, m): (usize, usize) = (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    );
    let planet: Vec<Vec<i32>> = iter
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    let mut villages = Vec::new();
    for (i, line) in planet.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 1 {
                villages.push((i, j));
            }
        }
    }

    let mut min_distance = 100;
    for i in 0..n {
        for j in 0..m {
            if planet[i][j] == 1 {
                continue;
            }

            // 1번 좌표 (i, j)
            for k in i..n {
                for l in j..m {
                    if planet[k][l] == 1 {
                        continue;
                    }

                    // 2번 좌표 (k, l)
                    let mut max_distance = 0;
                    for village in villages.iter() {
                        max_distance = max_distance.max(min(
                            calc_distance((i, j), *village),
                            calc_distance((k, l), *village),
                        ));
                    }

                    min_distance = min_distance.min(max_distance);
                }
            }
        }
    }

    println!("{}", min_distance);
}

fn calc_distance(point_a: (usize, usize), point_b: (usize, usize)) -> usize {
    point_a.0.abs_diff(point_b.0) + point_a.1.abs_diff(point_b.1)
}
