use std::io::{stdin, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    iter.next();
    iter.next();
    let b: i32 = iter.next().unwrap();
    let mut v: [i32; 256 + 1] = [0; 256 + 1];
    for i in iter {
        v[i as usize] += 1;
    }

    let mut result_time = i32::MAX;
    let mut result_height = 0;

    for height in 0..=256 {
        let mut time = 0;
        let mut bag = b;
        for i in 0..=256 {
            if v[i as usize] == 0 {
                continue;
            }
            match height.cmp(&i) {
                std::cmp::Ordering::Less => {
                    let blocks = (i - height) * v[i as usize];
                    bag += blocks;
                    time += blocks * 2;
                }
                std::cmp::Ordering::Equal => continue,
                std::cmp::Ordering::Greater => {
                    let blocks = (height - i) * v[i as usize];
                    bag -= blocks;
                    time += blocks;
                }
            }
        }
        if bag < 0 {
            continue;
        }

        if time <= result_time {
            result_time = time;
            result_height = height.max(result_height);
        }
    }

    println!("{} {}", result_time, result_height);
}
