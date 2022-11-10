use std::io::Write;
use std::{
    collections::HashSet,
    io::{stdin, stdout, BufWriter, Read},
};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n = iter.next().unwrap().parse().unwrap();
    iter.next();

    let mut n_set = HashSet::new();
    for _ in 0..n {
        n_set.insert(iter.next().unwrap());
    }
    let mut m_set = HashSet::new();
    for i in iter {
        m_set.insert(i);
    }

    let mut intersection: Vec<&str> = n_set.intersection(&m_set).copied().collect();

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);
    intersection.sort_unstable();
    writeln!(out, "{}", intersection.len()).unwrap();
    writeln!(out, "{}", intersection.join("\n")).unwrap();
}
