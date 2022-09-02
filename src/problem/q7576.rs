use std::{
    collections::VecDeque,
    io::{stdin, Read},
};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let (width, height) = (iter.next().unwrap() as usize, iter.next().unwrap() as usize);

    let mut tomato_box = Vec::with_capacity(height as usize);
    let mut queue = VecDeque::new();
    for x in 0..height {
        let mut row = Vec::with_capacity(width as usize);
        for y in 0..width {
            match iter.next().unwrap() {
                1 => {
                    row.push(Some(true));
                    queue.push_back((x as usize, y as usize));
                }
                0 => row.push(Some(false)),
                _ => row.push(None),
            }
        }
        tomato_box.push(row);
    }

    let mut count = -1;
    while !queue.is_empty() {
        count += 1;
        for _ in 0..queue.len() {
            let (x, y) = queue.pop_front().unwrap();

            let mut next = Vec::with_capacity(4);
            if x > 0 {
                next.push((x - 1, y));
            }
            if y > 0 {
                next.push((x, y - 1));
            }
            if x < height - 1 {
                next.push((x + 1, y));
            }
            if y < width - 1 {
                next.push((x, y + 1));
            }

            for (next_x, next_y) in next {
                if let Some(false) = tomato_box[next_x][next_y] {
                    tomato_box[next_x][next_y] = Some(true);
                    queue.push_back((next_x, next_y));
                }
            }
        }
    }
    for row in tomato_box {
        for tomato in row.into_iter().flatten() {
            if !tomato {
                count = -1;
                break;
            }
        }
    }

    println!("{}", count);
}
