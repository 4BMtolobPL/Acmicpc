use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut v = vec![0; 1000001];
    v[2] = 1;
    v[3] = 1;
    for index in 4..=1000000 {
        let mut min = v[index - 1];
        if index % 3 == 0 {
            min = min.min(v[index / 3]);
        }
        if index % 2 == 0 {
            min = min.min(v[index / 2]);
        }

        v[index] = min + 1;
    }

    println!("{}", v[n]);
}
