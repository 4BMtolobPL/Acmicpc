use std::io;
use std::io::stdin;

fn main() {
    let buf = io::read_to_string(stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let _n = iter.next().unwrap();
    let k: i32 = iter.next().unwrap();

    let v: Vec<_> = iter.collect();
    let mut min = i32::MAX;

    'outer: for (index, i) in v.iter().enumerate() {
        let mut count = 0;

        for (j_index, j) in v.iter().enumerate() {
            let estimated_diff = k * (j_index as i32 - index as i32);
            let actual_diff = j - i;
            if *i + estimated_diff <= 0 {
                continue 'outer;
            }
            if actual_diff != estimated_diff {
                count += 1;
            }
        }

        min = min.min(count);
    }

    println!("{}", min);
}
