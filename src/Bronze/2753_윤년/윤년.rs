use std::io;

fn main() {
    let mut buf  = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let year: i32 = buf.trim().parse().unwrap();

    if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
        println!("1");
    }
    else {
        println!("0");
    }
}