use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    let (n, m) = {
        let mut line = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap());
        (line.next().unwrap(), line.next().unwrap())
    };

    let players: Vec<_> = lines
        .map(|line| {
            let mut cards: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            cards.sort();
            cards
        })
        .collect();

    let mut wins = vec![0; n];
    for i in 0..m {
        let v: Vec<(usize, i32)> = players.iter().map(|x| x[i]).enumerate().collect();
        let max = v.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

        v.iter()
            .filter(|x| x.1 == max.1)
            .for_each(|x| wins[x.0] += 1)
    }

    let winner_score = wins.iter().max().unwrap();

    println!(
        "{}",
        wins.iter()
            .enumerate()
            .filter(|x| x.1 == winner_score)
            .map(|x| (x.0 + 1).to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
