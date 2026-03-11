use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let n = iter.next().unwrap();
    let t = iter.next().unwrap();

    let mut v: Vec<(i32, i32)> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        v.push((iter.next().unwrap(), iter.next().unwrap()));
    }

    v.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut sum: i64 = 0;

    let mut prev_shadow = *v.first().unwrap();
    for (position, height) in v.iter().skip(1) {
        if let Some(diff) = t.checked_mul(position - prev_shadow.0) {
            let shadow = prev_shadow.1 - diff;

            if shadow > 0 {
                if shadow >= *height {
                    sum += *height as i64;
                    prev_shadow = (*position, shadow);
                    continue;
                } else {
                    sum += shadow as i64;
                }
            }
        }
        prev_shadow = (*position, *height);
    }

    println!("{sum}");
}
