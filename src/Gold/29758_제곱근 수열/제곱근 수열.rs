use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    let mut v: Vec<Vec<i32>> = vec![vec![]; 6];
    v[1] = vec![0; 70001];
    v[1][1] = 1;

    'outer: for i in 2..=5 {
        v[i].push(0);

        let mut sum = 0;
        for j in 0.. {
            sum += v[i - 1][j];
            for k in (j * j + 1)..=((j + 1) * (j + 1)) {
                v[i].push(sum);
                if k >= 70000 {
                    continue 'outer;
                }
            }
        }
    }

    let mut out = String::new();
    for line in buf.lines().skip(1) {
        let mut iter = line.split_ascii_whitespace().map(|x| x.parse().unwrap());
        let n: usize = iter.next().unwrap();
        let l = iter.next().unwrap();

        if l > 5 {
            writeln!(&mut out, "0").unwrap();
        } else {
            writeln!(&mut out, "{}", v[l][n]).unwrap();
        }
    }

    print!("{out}");
}
