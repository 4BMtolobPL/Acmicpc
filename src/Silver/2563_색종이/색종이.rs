use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let lines = buf.lines();

    let mut v: Vec<Vec<i32>> = vec![vec![0; 100]; 100];
    for line in lines.skip(1) {
        let mut iter = line.split_ascii_whitespace().map(|x| x.parse().unwrap());

        let left: usize = iter.next().unwrap();
        let top = iter.next().unwrap();
        let right = left + 10;
        let bottom = top + 10;

        v[top][left] += 1;
        if bottom < 100 {
            v[bottom][left] -= 1;
        }
        if right < 100 {
            v[top][right] -= 1;
        }
        if bottom < 100 && right < 100 {
            v[bottom][right] += 1;
        }
    }

    for i in 0..100 {
        let mut sum = 0;
        for j in 0..100 {
            sum += v[i][j];
            v[i][j] = if i > 0 { sum + v[i - 1][j] } else { sum };
        }
    }

    println!("{}", v.iter().flatten().filter(|x| **x > 0).count());
}
