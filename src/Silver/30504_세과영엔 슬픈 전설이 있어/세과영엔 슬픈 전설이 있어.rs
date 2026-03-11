use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();
    let _n = lines.next().unwrap();
    let a_collection: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let b_collection: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut map = BTreeMap::new();
    for b in b_collection {
        if let Entry::Vacant(x) = map.entry(b) {
            x.insert(1);
        } else {
            let p = map.get_mut(&b).unwrap();
            *p += 1;
        }
    }

    let mut out = String::new();
    for a in a_collection {
        let closest_greater = map.range_mut(a..).next();

        if let Some((&key, value)) = closest_greater {
            write!(&mut out, "{} ", key).unwrap();

            *value -= 1;
            if *value <= 0 {
                map.remove(&key);
            }
        } else {
            out = String::from("-1");
            break;
        }
    }

    println!("{}", out);
}
