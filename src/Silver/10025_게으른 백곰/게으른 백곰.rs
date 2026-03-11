use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());

    let n: usize = iter.next().unwrap();
    let k = iter.next().unwrap();

    let mut v = vec![0; 1000001];
    for _ in 0..n {
        let ice = iter.next().unwrap();
        let position = iter.next().unwrap();
        v[position] = ice;
    }

    let mut lhs = 0;
    let mut rhs = k * 2 + 1;
    let mut sum: usize = v.iter().take(rhs).sum();
    let mut max = sum;

    while rhs <= 1000000 {
        sum -= v[lhs];
        sum += v[rhs];
        max = max.max(sum);

        lhs += 1;
        rhs += 1;
    }

    println!("{max}");
}
