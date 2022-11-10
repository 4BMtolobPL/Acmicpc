use std::io::Write;
use std::io::{stdin, stdout, BufWriter, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let (n, m) = (iter.next().unwrap() as usize, iter.next().unwrap() as usize);
    let mut v = vec![vec![0; m]; n];

    for _ in 0..2 {
        for i in v.iter_mut() {
            for j in i.iter_mut() {
                *j += iter.next().unwrap();
            }
        }
    }

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);

    for i in v {
        for j in i {
            write!(out, "{} ", j).unwrap();
        }
        writeln!(out).unwrap();
    }
}
