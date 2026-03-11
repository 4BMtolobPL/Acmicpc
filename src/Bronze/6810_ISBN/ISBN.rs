use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    println!(
        "The 1-3-sum is {}",
        buf.split_whitespace()
            .enumerate()
            .map(|(index, x)| {
                let x: i32 = x.parse().unwrap();
                if index % 2 == 0 {
                    x
                } else {
                    x * 3
                }
            })
            .sum::<i32>()
            + 91
    );
}