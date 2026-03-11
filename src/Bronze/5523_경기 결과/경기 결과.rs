use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let lines = buf.lines();

    let scores: Vec<(i32, i32)> = lines
        .skip(1)
        .map(|x| {
            if let Some((a, b)) = x.split_once(" ") {
                return (a.parse().unwrap(), b.parse().unwrap());
            };
            (0, 0)
        })
        .collect();

    println!(
        "{} {}",
        scores.iter().filter(|(a, b)| a > b).count(),
        scores.iter().filter(|(a, b)| a < b).count()
    );
}
