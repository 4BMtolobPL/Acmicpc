use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    let (n, _h, w): (usize, _, _) = {
        let mut words = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap());
        (
            words.next().unwrap(),
            words.next().unwrap(),
            words.next().unwrap(),
        )
    };

    let note: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut out = String::new();
    'outer: for i in 0..n {
        for row_slice in note.iter() {
            for k in row_slice.iter().skip(i * w).take(w) {
                if *k == '?' {
                    continue;
                } else {
                    out.push(*k);
                    continue 'outer;
                }
            }
        }
        out.push('?');
    }

    println!("{}", out);
}
