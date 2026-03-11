use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let (x, y): (i32, i32) = {
        let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    if x > y {
        println!("{}", x + y + (y / 10));
    } else {
        println!("{}", y + x + (x / 10));
    }
}
