use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let m: Vec<Vec<&str>> = buf
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();

    println!("{}", is_set(&m))
}

fn is_set(m: &Vec<Vec<&str>>) -> i32 {
    for i in 0..10 {
        let left = m[i][0];
        let top = m[0][i];

        if m[i].iter().all(|x| *x == left) || m.iter().map(|x| x[i]).all(|x| x == top) {
            return 1;
        }
    }

    0
}
