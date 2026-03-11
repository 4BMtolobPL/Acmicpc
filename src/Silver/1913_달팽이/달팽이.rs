use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let n = iter.next().unwrap();
    let target = iter.next().unwrap();

    let mut v = vec![vec![0; n]; n];
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut direction = 0;
    let mut x = n / 2;
    let mut y = n / 2;
    let mut count = 2;
    v[x][y] = 1;
    let max_count = n * n;
    let mut find_point = (x, y);

    'outer: for distance in 1.. {
        for _ in 0..2 {
            let (dx, dy) = directions[direction];
            for _ in 0..distance {
                if count > max_count {
                    break 'outer;
                }

                x = (x as isize + dx) as usize;
                y = (y as isize + dy) as usize;
                v[x][y] = count;

                if count == target {
                    find_point = (x, y);
                }
                count += 1;
            }
            direction = (direction + 1) % 4;
        }
    }

    let mut out = String::with_capacity((max_count.to_string().len() + 1) * max_count);
    for row in v.iter() {
        let mut iter = row.iter();
        write!(out, "{}", iter.next().unwrap()).unwrap();
        for i in iter {
            write!(out, " {}", i).unwrap();
        }
        writeln!(out).unwrap();
    }
    writeln!(out, "{} {}", find_point.0 + 1, find_point.1 + 1).unwrap();

    print!("{}", out);
}
