use std::fmt::Write;
use std::io::{Read, stdin};

const MAX_FREQUENCY: i32 = 146000;
const MIN_FREQUENCY: i32 = 144000;

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let lines = buf.lines();

    let mut out = String::new();
    for line in lines.skip(1) {
        let mut iter = line.split_whitespace();
        let a_frequency = parse_frequency(iter.next().unwrap());
        let b_frequency = parse_frequency(iter.next().unwrap());
        let channel = iter.next().unwrap();
        let target_frequency = parse_frequency(iter.next().unwrap());

        let mut v = Vec::new();
        v.push(6);

        if let Some(x) = up_button(a_frequency, target_frequency) {
            v.push(x + channel_switch("A", channel));
        }
        if let Some(x) = up_button(b_frequency, target_frequency) {
            v.push(x + channel_switch("B", channel));
        }
        if let Some(x) = down_button(a_frequency, target_frequency) {
            v.push(x + channel_switch("A", channel));
        }
        if let Some(x) = down_button(b_frequency, target_frequency) {
            v.push(x + channel_switch("B", channel));
        }

        writeln!(out, "{}", v.iter().min().unwrap()).unwrap();
    }

    print!("{}", out);
}

fn channel_switch(from: &str, to: &str) -> i32 {
    if from == to { 0 } else { 1 }
}

fn parse_frequency(s: &str) -> i32 {
    let mut t = String::new();
    for slice in s.split(".") {
        t.push_str(slice);
    }

    t.parse().unwrap()
}

fn up_button(from: i32, to: i32) -> Option<i32> {
    if from == to {
        Some(0)
    } else if from < to {
        return if (to - from) % 20 == 0 {
            Some((to - from) / 20)
        } else {
            None
        };
    } else {
        return if (to - MIN_FREQUENCY) % 20 == 0 {
            Some((MAX_FREQUENCY - from) / 20 + 1 + (to - MIN_FREQUENCY) / 20)
        } else {
            None
        };
    }
}

fn down_button(from: i32, to: i32) -> Option<i32> {
    if from == to {
        Some(0)
    } else if to < from {
        return if (from - to) % 20 == 0 {
            Some((from - to) / 20)
        } else {
            None
        };
    } else {
        return if (MAX_FREQUENCY - to) % 20 == 0 {
            Some((from - MIN_FREQUENCY) / 20 + 1 + (MAX_FREQUENCY - to) / 20)
        } else {
            None
        };
    }
}
