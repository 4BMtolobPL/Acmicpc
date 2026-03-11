use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let n: usize = iter.next().unwrap();

    let mut count = vec![usize::MAX];
    for i in iter {
        if *count.last().unwrap() > i {
            count.push(i);
        } else {
            let index = count.partition_point(|x| *x > i);
            count[index] = i;
        }
    }

    println!("{}", n - (count.len() - 1));
}
