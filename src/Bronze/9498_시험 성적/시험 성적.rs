use std::io;

fn main() {
    let mut buf  = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let score: i32 = buf.trim().parse().unwrap();

    match score / 10 {
        10 => println!("A"),
        9 => println!("A"),
        8 => println!("B"),
        7 => println!("C"),
        6 => println!("D"),
        _ => println!("F"),
    }
}