use std::cmp::min;
use std::collections::BTreeMap;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let _n = iter.next().unwrap();

    let mut map: BTreeMap<&str, Vec<i32>> = BTreeMap::new();
    while let (Some(index), Some(color)) = (iter.next(), iter.next()) {
        map.entry(color).or_default().push(index.parse().unwrap());
    }

    let mut sum = 0;
    for group in map.values_mut() {
        group.sort_unstable();

        for i in 0..group.len() {
            if i == 0 {
                sum += group[i + 1] - group[i];
            } else if i == group.len() - 1 {
                sum += group[i] - group[i - 1];
            } else {
                sum += min(group[i + 1] - group[i], group[i] - group[i - 1]);
            }
        }
    }

    println!("{}", sum);
}
