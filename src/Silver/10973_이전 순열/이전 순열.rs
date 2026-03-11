use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut v: Vec<i32> = iter.map(|x| x.parse().unwrap()).collect();

    let mut index = None;
    for i in (1..n).rev() {
        if v[i - 1] > v[i] {
            index = Some(i);
            break;
        }
    }

    if let Some(i) = index {
        let mut index = i;
        for j in (i..n).rev() {
            if v[j] < v[i - 1] {
                index = j;
                break;
            }
        }
        v.swap(i - 1, index);
        v[i..n].reverse();
        println!(
            "{}",
            v.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    } else {
        println!("-1");
    }
}
