use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let iter = buf
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap());

    let mut is_winner = vec![
        false, true, false, true, true, false, true, false, true, true,
    ];

    let mut out = String::new();
    for n in iter {
        for i in is_winner.len()..=n {
            if (1..)
                .take_while(|x| x * x <= i)
                .any(|x| !is_winner[i - (x * x)])
            {
                is_winner.push(true);
            } else {
                is_winner.push(false);
            }
        }

        if is_winner[n] {
            writeln!(out, "koosaga").unwrap();
        } else {
            writeln!(out, "cubelover").unwrap();
        }
    }

    print!("{out}");
}
