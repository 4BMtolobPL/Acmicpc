use std::{
    collections::VecDeque,
    io::{stdin, Read},
};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    let (m, n, h): (u8, _, _) = {
        let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
        (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };

    buf.clear();
    stdin().read_to_string(&mut buf).unwrap();
    let mut tomato = buf.split_whitespace();

    let mut tomato_box = vec![vec![vec![true; m as usize]; n as usize]; h as usize];
    let mut queue = VecDeque::new();
    let mut unripe = 0;
    for (z, layer) in tomato_box.iter_mut().enumerate() {
        for (y, row) in layer.iter_mut().enumerate() {
            for (x, value) in row.iter_mut().enumerate() {
                match tomato.next().unwrap() {
                    "1" => queue.push_back((z as u8, y as u8, x as u8)),
                    "0" => {
                        *value = false;
                        unripe += 1;
                    }
                    "-1" => {}
                    _ => unreachable!(),
                }
            }
        }
    }

    let mut count = -1;
    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let (z, y, x) = queue.pop_front().unwrap();

            let mut next_closer = |next_z, next_y, next_x| {
                let next_position = tomato_box
                    .get_mut(next_z as usize)
                    .unwrap()
                    .get_mut(next_y as usize)
                    .unwrap()
                    .get_mut(next_x as usize)
                    .unwrap();
                if !*next_position {
                    *next_position = true;
                    queue.push_back((next_z, next_y, next_x));
                    unripe -= 1;
                }
            };

            if z > 0 {
                next_closer(z - 1, y, x);
            }
            if z < h - 1 {
                next_closer(z + 1, y, x);
            }
            if y > 0 {
                next_closer(z, y - 1, x);
            }
            if y < n - 1 {
                next_closer(z, y + 1, x);
            }
            if x > 0 {
                next_closer(z, y, x - 1);
            }
            if x < m - 1 {
                next_closer(z, y, x + 1);
            }
        }
        count += 1;
    }

    if unripe > 0 {
        println!("-1");
    } else {
        println!("{}", count);
    }
}
