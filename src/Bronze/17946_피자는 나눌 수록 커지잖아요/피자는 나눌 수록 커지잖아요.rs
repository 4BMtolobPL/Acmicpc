use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    let test_cases = lines.next().unwrap().parse().unwrap();

    print!("{}", "1\n".repeat(test_cases))
}
