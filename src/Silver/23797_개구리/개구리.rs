use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    let mut k = 0;
    let mut p = 0;

    for c in buf.trim_end().chars() {
        if c == 'K' {
            if p > 0 {
                p -= 1;
            }

            k += 1;
        } else if c == 'P' {
            if k > 0 {
                k -= 1;
            }

            p += 1;
        }
    }

    println!("{}", k + p);
}
