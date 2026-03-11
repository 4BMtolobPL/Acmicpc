use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();
    let test_case: i32 = lines.next().unwrap().parse().unwrap();

    'outer: for _ in 0..test_case {
        let n: usize = lines.next().unwrap().parse().unwrap();
        let v: Vec<usize> = lines.by_ref().take(n).map(|x| x.parse().unwrap()).collect();

        let mut count = 0;
        let mut position = 0;
        for _ in 0..n {
            position = v[position] - 1;
            count += 1;

            if position == n - 1 {
                println!("{}", count);
                continue 'outer;
            } else if position == 0 {
                println!("0");
                continue 'outer;
            }
        }

        println!("0");
    }
}
