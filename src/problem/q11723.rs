use std::io::stdin;
use std::io::stdout;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let m = buf.trim().parse().unwrap();

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);

    let mut set = 0;
    for _ in 0..m {
        buf.clear();
        stdin().read_line(&mut buf).unwrap();
        let mut words = buf.split_whitespace();
        match words.next().unwrap() {
            "add" => {
                let x = words.next().unwrap().parse().unwrap();
                add(&mut set, x);
            }
            "remove" => {
                let x = words.next().unwrap().parse().unwrap();
                remove(&mut set, x);
            }
            "check" => {
                let x = words.next().unwrap().parse().unwrap();
                writeln!(out, "{}", check(&mut set, x)).unwrap();
            }
            "toggle" => {
                let x = words.next().unwrap().parse().unwrap();
                toggle(&mut set, x);
            }
            "all" => all(&mut set),
            "empty" => empty(&mut set),
            _ => continue,
        }
    }
}

fn add(set: &mut i32, x: usize) {
    *set |= 1 << x;
}

fn remove(set: &mut i32, x: usize) {
    *set &= !(1 << x);
}

fn check(set: &mut i32, x: usize) -> u8 {
    if *set & (1 << x) == 0 {
        0
    } else {
        1
    }
}

fn toggle(set: &mut i32, x: usize) {
    *set ^= 1 << x;
}

fn all(set: &mut i32) {
    *set = !0;
}

fn empty(set: &mut i32) {
    *set = 0;
}
