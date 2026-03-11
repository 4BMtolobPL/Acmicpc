use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let (_r, c): (usize, _) = {
        let mut iter = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    let v: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut first = None;
    for line in v.iter() {
        for chars in line.split(|x| *x == '#') {
            if chars.len() < 2 {
                continue;
            }

            let word = String::from_iter(chars);
            if let Some(ref f) = first {
                if word < *f {
                    first = Some(word);
                }
            } else {
                first = Some(word);
            }
        }
    }

    for i in 0..c {
        let col: Vec<char> = v.iter().map(|x| x[i]).collect();

        for chars in col.split(|x| *x == '#') {
            if chars.len() < 2 {
                continue;
            }

            let word = String::from_iter(chars);
            if let Some(ref f) = first {
                if word < *f {
                    first = Some(word);
                }
            } else {
                first = Some(word);
            }
        }
    }

    println!("{}", first.unwrap());
}
