use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines();

    let plain = lines.next().unwrap().trim_end();
    let decrypted = lines.next().unwrap().trim_end();

    let mut v = Vec::new();
    for (a, b) in plain.bytes().zip(decrypted.bytes()) {
        if b > a {
            v.push(b - a + b'A' - 1);
        } else {
            v.push(b + 26 - a + b'A' - 1);
        }
    }

    println!("{}", String::from_utf8_lossy(get_shortest(&v)));
}

fn get_shortest(v: &[u8]) -> &[u8] {
    let end = v.last().unwrap();

    'outer: for i in 1..=v.len() {
        if v.len() % i != 0 {
            continue 'outer;
        }

        let slice = &v[0..i];
        if slice.last().unwrap() != end {
            continue 'outer;
        }

        for j in (i..v.len()).step_by(i) {
            if slice != &v[j..j + i] {
                continue 'outer;
            }
        }

        return slice;
    }

    v
}
