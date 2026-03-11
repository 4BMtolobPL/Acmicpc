use std::io;
use std::io::stdin;

fn main() {
    let buf = io::read_to_string(stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let maze: Vec<_> = iter.map(|line| line.find('0').unwrap()).collect();
    
    let min: usize = (0..4)
        .map(|i| {
            maze.iter()
                .map(|x| {
                    let diff = x.abs_diff(i);
                    match diff {
                        3 => 1,
                        _ => diff,
                    }
                })
                .sum()
        })
        .min()
        .unwrap();

    println!("{}", min + n + 2);
}
