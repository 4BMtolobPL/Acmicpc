use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let (n, _m): (usize, usize) = {
        let mut iter = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    let mut sum_array = Vec::with_capacity(n);

    {
        let mut sum: i32 = 0;
        let mut first_line = Vec::with_capacity(n);
        for i in lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
        {
            sum += i;
            first_line.push(sum);
        }
        sum_array.push(first_line);
    }

    for (index_i, line) in lines.by_ref().take(n - 1).enumerate() {
        let mut sum = 0;
        let mut v = Vec::with_capacity(n);
        for (index_j, j) in line
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .enumerate()
        {
            sum += j;
            v.push(sum + sum_array[index_i][index_j]);
        }
        sum_array.push(v);
    }

    let mut out = String::new();
    for line in lines {
        let (x1, y1, x2, y2) = {
            let mut iter = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<usize>().unwrap());
            (
                iter.next().unwrap() - 1,
                iter.next().unwrap() - 1,
                iter.next().unwrap() - 1,
                iter.next().unwrap() - 1,
            )
        };
        let mut sum = sum_array[x2][y2];
        if x1 > 0 {
            sum -= sum_array[x1 - 1][y2];
        }
        if y1 > 0 {
            sum -= sum_array[x2][y1 - 1];
        }
        if x1 > 0 && y1 > 0 {
            sum += sum_array[x1 - 1][y1 - 1];
        }

        writeln!(out, "{sum}").unwrap();
    }

    print!("{out}");
}
