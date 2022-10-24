use std::{
    collections::VecDeque,
    io::{stdin, Read},
};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    let (m, n, h): (usize, _, _) = {
        let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
        (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap()
        )
    };

    buf.clear();
    stdin().read_to_string(&mut buf).unwrap();
    let mut tomato = buf.split_whitespace();

    let mut tomato_box = vec![vec![vec![true; 100]; 100]; 100];
    let mut stack = VecDeque::new();
    let mut unripe = 0;
    for z in 0..h {
        for y in 0..n {
            for x in 0..m {
                match tomato.next().unwrap() {
                    "1" => {
                        // tomato_box[z][y][x] = true;
                        stack.push_back((z, y, x));
                    }
                    "0" => {
                        tomato_box[z][y][x] = false;
                        unripe += 1;
                    }
                    "-1" => {
                        // tomato_box[z][y][x] = true;
                    },
                    _ => unreachable!(),
                }
            }
        }
    }

    let mut count = -1;
    while !stack.is_empty() {
        // println!("{:?}", stack);
        for _ in 0..stack.len() {
            let (z, y, x) = stack.pop_front().unwrap();

            let mut next = Vec::new();
            if z > 0 {
                next.push((z - 1, y, x));
            }
            if z < h - 1 {
                next.push((z + 1, y, x));
            }
            if y > 0 {
                next.push((z, y - 1, x));
            }
            if y < n - 1 {
                next.push((z, y + 1, x));
            }
            if x > 0 {
                next.push((z, y, x - 1));
            }
            if x < m - 1 {
                next.push((z, y, x + 1));
            }

            for (next_z, next_y, next_x) in next {
                if !tomato_box[next_z][next_y][next_x] {
                    tomato_box[next_z][next_y][next_x] = true;
                    stack.push_back((next_z, next_y, next_x));
                    unripe -= 1;
                }
            }
        }
        count += 1;
    }

    if unripe > 0 {
        count = -1
    }
    println!("{}", count);
}
