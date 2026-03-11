use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines();

    let p: usize = lines.next().unwrap().parse().unwrap();
    let mut out = String::new();
    for _ in 0..p {
        let bytes: usize = lines.next().unwrap().parse().unwrap();
        let mut s = String::new();

        while s.len() < (bytes * 2) {
            let line = lines.next().unwrap();
            s.push_str(line);
        }

        let mut decode_data = String::new();
        let mut total_decode_bytes = 0;

        let mut index = 0;
        while index < s.len() {
            let b = hex_to_u8(&s[index..index + 2]);
            index += 2;

            if is_count(b) {
                let repeats = get_7_bits(b) + 3;
                let values = &s[index..index + 2];
                index += 2;

                write!(decode_data, "{}", values.repeat(repeats)).unwrap();
                total_decode_bytes += repeats;
            } else {
                let n = get_7_bits(b) + 1;
                let values = &s[index..index + (n * 2)];

                index += n * 2;

                write!(decode_data, "{values}").unwrap();
                total_decode_bytes += n;
            }
        }

        writeln!(out, "{total_decode_bytes}").unwrap();
        for i in (0..decode_data.len()).step_by(80) {
            writeln!(out, "{}", &decode_data[i..(i + 80).min(decode_data.len())]).unwrap();
        }
    }

    print!("{out}");
}

fn hex_to_u8(s: &str) -> u8 {
    u8::from_str_radix(s, 16).unwrap()
}

fn is_count(b: u8) -> bool {
    b >> 7 == 1
}

fn get_7_bits(b: u8) -> usize {
    let r = b & (0xFF >> 1);
    r as usize
}
