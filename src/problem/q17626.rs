use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut v = [0; 50001];

    for index in 1..=50000 {
        let mut min = 5;
        for sqrt in 1..233 {
            let square = sqrt * sqrt;
            if square > index {
                break;
            }

            min = min.min(v[index - square]);
        }
        v[index] = min + 1;
    }

    println!("{}", v[n]);
}
