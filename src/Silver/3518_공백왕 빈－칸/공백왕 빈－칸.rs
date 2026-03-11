use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let lines: Vec<Vec<_>> = buf
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let mut word_len: Vec<_> = vec![0; 90];
    for words in lines.iter() {
        for i in 0..words.len() {
            word_len[i] = word_len[i].max(words[i].len())
        }
    }

    let mut out = String::new();
    for words in lines.iter() {
        let mut s = String::new();
        let mut iter = words.iter().enumerate();
        let (_, first) = iter.next().unwrap();
        write!(s, "{:<width$}", first, width = word_len[0]).unwrap();
        for (index, word) in iter {
            write!(s, " {:<width$}", word, width = word_len[index]).unwrap();
        }
        writeln!(out, "{}", s.trim_end()).unwrap();
    }

    print!("{}", out);
}
