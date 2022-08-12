use std::io::{stdin, BufReader, BufRead, BufWriter, stdout};
use std::io::prelude::*;

#[allow(dead_code)]
pub fn solve() {
    let stdin = stdin();
    let stdout = stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let mut v = Vec::new();

    for _ in 0..n {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        
        match iter.next().unwrap() {
            "push" => {v.push(iter.next().unwrap().to_string());},
            "pop" => {writeln!(writer, "{}", pop(&mut v)).unwrap();},
            "size" => {writeln!(writer, "{}", v.len()).unwrap();},
            "empty" => {writeln!(writer, "{}", if v.is_empty() {1} else {0}).unwrap();},
            "top" => {writeln!(writer, "{}", top(&v)).unwrap();},
            _ => {},
        };
    }
}

fn pop(v: &mut Vec<String>) -> String {
    match v.pop() {
        Some(x) => x,
        None => "-1".to_string(),
    }
}

fn top(v: &Vec<String>) -> &str {
    match v.last() {
        Some(x) => x,
        None => "-1",
    }
}