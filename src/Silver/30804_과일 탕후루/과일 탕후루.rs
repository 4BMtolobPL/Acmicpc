use std::cmp::max;
use std::collections::BTreeSet;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    let n = iter.next().unwrap();

    if n == 1 {
        println!("1");
        return;
    }

    let mut tanghuru: Vec<_> = vec![];
    tanghuru.reserve_exact(n);

    let mut t = (iter.next().unwrap(), 1);
    for i in iter {
        if t.0 == i {
            t.1 += 1;
        } else {
            tanghuru.push(t);
            t = (i, 1);
        }
    }
    tanghuru.push(t);

    let mut max_tanghuru = 0;
    let mut lhs = 0;
    let mut rhs = 1;
    let mut fruit_types = BTreeSet::new();
    fruit_types.insert(tanghuru[lhs].0);

    while lhs < rhs && rhs < tanghuru.len() {
        fruit_types.insert(tanghuru[rhs].0);
        if fruit_types.len() > 2 {
            max_tanghuru = max(max_tanghuru, tanghuru[lhs..rhs].iter().map(|x| x.1).sum());

            lhs = rhs - 1;
            fruit_types.clear();
            fruit_types.insert(tanghuru[lhs].0);
            fruit_types.insert(tanghuru[rhs].0);
        }

        rhs += 1;
    }
    max_tanghuru = max(max_tanghuru, tanghuru[lhs..rhs].iter().map(|x| x.1).sum());

    println!("{}", max_tanghuru);
}
