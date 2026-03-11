use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let iter = buf.lines().skip(1).map(|line| {
        let mut iter = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    });

    let mut max = 0;
    let mut music_len = 0;
    let mut download_len = 0;
    for (music, download) in iter {
        download_len += download;
        max = max.max(download_len - music_len);
        music_len += music;
    }

    println!("{max}");
}
