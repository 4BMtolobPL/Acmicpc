use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for slice in buf.split("#") {
        let x = slice.trim();

        if x.is_empty() {
            break;
        } else if is_ladder(x) {
            println!("Correct");
        } else {
            println!("Incorrect");
        }
    }
}

fn is_ladder(s: &str) -> bool {
    let mut prev: Option<&str> = None;
    for line in s.lines() {
        if let Some(p) = prev {
            if p.len() != line.len()
                || p.chars().zip(line.chars()).filter(|(a, b)| a != b).count() != 1
            {
                return false;
            }
        }

        prev = Some(line);
    }

    true
}
