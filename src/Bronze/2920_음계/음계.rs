use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let s = buf.trim();
    let result;

    if s == "1 2 3 4 5 6 7 8" {
        result = "ascending";
    } else if s == "8 7 6 5 4 3 2 1" {
        result = "descending";
    } else {
        result = "mixed";
    }

    println!("{}", result);
}