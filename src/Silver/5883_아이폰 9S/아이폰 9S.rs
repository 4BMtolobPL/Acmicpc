use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let v: Vec<i32> = buf
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let mut max = 0;
    'outer: for i in 0..v.len() {
        let lhs = v[i];
        let mut rhs = None;
        let mut count = 1;
        for j in v.iter().skip(i + 1) {
            if lhs == *j {
                count += 1;
                continue;
            }

            if let Some(r) = rhs {
                if r != *j {
                    max = max.max(count);
                    continue 'outer;
                }
            } else {
                rhs = Some(*j);
            }
        }

        max = max.max(count);
    }

    println!("{max}");
}
