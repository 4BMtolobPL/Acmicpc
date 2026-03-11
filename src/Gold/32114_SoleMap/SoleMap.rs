use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines();
    let (n, _m): (usize, usize) = {
        let mut iter = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    let road_lanes: Vec<i64> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut d_array: Vec<i64> = vec![0; n];

    for line in lines {
        let mut iter = line.split_ascii_whitespace();
        let u: usize = iter.next().unwrap().parse().unwrap();
        let v: usize = iter.next().unwrap().parse().unwrap();
        let x: i64 = iter.next().unwrap().parse().unwrap();

        d_array[u - 1] += x;
        d_array[v - 1] -= x;
    }

    let mut traffic_volume = Vec::new();
    let mut sum = 0;
    for val in d_array.iter().take(n - 1) {
        sum += val;
        traffic_volume.push(sum);
    }

    let mut out = String::new();
    for (lane, traffic) in road_lanes.iter().zip(traffic_volume.iter()) {
        let d = traffic / lane;
        let mo = traffic % lane;

        writeln!(out, "{}", lane * d * d + mo * (d * 2 + 1)).unwrap();
    }

    print!("{out}")
}
