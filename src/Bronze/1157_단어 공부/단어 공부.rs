use std::io;

fn main() {
    let mut buf  = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let text = buf.trim().to_ascii_lowercase();

    let mut v: [usize; 26] = [0; 26];

    for word in text.as_bytes() {
        v[(word - b'a') as usize] += 1;
    }

    let mut max = 0;
    let mut out: char = '?';

    for i in 0..v.len() {
        if v[i] == max {
            out = '?';
        }
        else if v[i] > max {
            max = v[i];
            out = (b'A' + i as u8) as char;
        }
    }
    println!("{}", out);
}