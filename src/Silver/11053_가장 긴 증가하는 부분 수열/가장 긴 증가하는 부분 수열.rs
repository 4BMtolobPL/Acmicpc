use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let n: usize = iter.next().unwrap();
    let progression: Vec<_> = iter.collect();
    let mut counts = Vec::with_capacity(n);
    let mut memos: Vec<_> = Vec::with_capacity(n);

    counts.push(1);
    memos.push(*progression.first().unwrap());

    for &i in progression.iter().skip(1) {
        if *memos.last().unwrap() < i {
            memos.push(i);
            counts.push(memos.len());
        } else {
            let index = memos.partition_point(|&x| x < i);
            if memos[index] > i {
                memos[index] = i;
            }
            counts.push(index + 1);
        }
    }

    println!("{}", counts.iter().max().unwrap());
}
