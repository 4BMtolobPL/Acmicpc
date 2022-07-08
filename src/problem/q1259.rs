#[allow(dead_code)]
pub fn solve() {
    let mut line = get_line();
    while line != "0" {
        if is_palindrome(&line) {
            println!("yes");
        } else {
            println!("no");
        }
        line = get_line();
    }
}

fn get_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn is_palindrome(line: &str) -> bool {
    let mut lh = line.chars();
    let mut rh = line.chars().rev();

    for _ in 0..(line.len() / 2) {
        if lh.next().unwrap() != rh.next().unwrap() {
            return false;
        }
    }

    if line.len() % 2 == 1 {
        if lh.next().unwrap() != rh.next().unwrap() {
            return false;
        }
    }

    return true;
}
