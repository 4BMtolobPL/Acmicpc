use std::collections::VecDeque;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let v: VecDeque<&str> = buf.split_ascii_whitespace().collect();

    if is_valid(v) {
        println!("Valid");
    } else {
        println!("Invalid");
    }
}

fn is_valid(mut v: VecDeque<&str>) -> bool {
    let position_to_point = |x: &str| {
        let bytes = x.as_bytes();
        ((bytes[0] - b'A') as usize, (bytes[1] - b'1') as usize)
    };
    let check_diff = |point1: (usize, usize), point2: (usize, usize)| {
        let a = point1.0.abs_diff(point2.0);
        let b = point1.1.abs_diff(point2.1);
        a + b == 3 && (a == 1 || a == 2)
    };

    let mut visited = vec![vec![false; 6]; 6];
    let first = v.pop_front().unwrap();
    let mut prev = position_to_point(first);
    v.push_back(first);

    for &i in v.iter() {
        let point = position_to_point(i);

        if !check_diff(prev, point) {
            return false;
        }

        if visited[point.0][point.1] {
            return false;
        } else {
            visited[point.0][point.1] = true;
        }

        prev = point;
    }

    true
}
