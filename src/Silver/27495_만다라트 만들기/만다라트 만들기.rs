use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let v: Vec<Vec<_>> = buf
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();

    let mut set: Vec<(&str, _)> = Vec::with_capacity(8);
    for i in (0..9).step_by(3) {
        for j in (0..9).step_by(3) {
            if i == 3 && j == 3 {
                continue;
            }

            let mut inner_v: Vec<&str> = Vec::with_capacity(8);
            let mut iter = v
                .get(i..i + 3)
                .unwrap()
                .iter()
                .flat_map(|x| x.get(j..j + 3).unwrap());

            iter.by_ref().take(4).for_each(|x| inner_v.push(x));
            let y = iter.next().unwrap();
            iter.by_ref().take(4).for_each(|x| inner_v.push(x));

            inner_v.sort_unstable();
            set.push((y, inner_v));
        }
    }

    set.sort_unstable_by(|lhs, rhs| lhs.0.cmp(rhs.0));

    let mut out = String::new();
    for (index_i, i) in set.iter().enumerate() {
        let x = index_i + 1;
        writeln!(out, "#{}. {}", x, i.0).unwrap();
        for (index_j, j) in i.1.iter().enumerate() {
            writeln!(out, "#{}-{}. {}", x, index_j + 1, j).unwrap();
        }
    }

    print!("{out}");
}
