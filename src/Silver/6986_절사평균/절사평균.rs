use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let mut v: Vec<f64> = iter.map(|x| x.parse().unwrap()).collect();
    v.sort_by(|a, b| {
        if a < b {
            Ordering::Less
        } else if a > b {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    println!(
        "{:.2}",
        (get_truncated_mean(n, k, &v) * 100.0).round() / 100.0
    );
    println!(
        "{:.2}",
        (get_adjusted_mean(n, k, &v) * 100.0).round() / 100.0
    );
}

fn get_truncated_mean(n: usize, k: usize, v: &[f64]) -> f64 {
    v.iter().skip(k).take(n - k - k).sum::<f64>() / (n - k - k) as f64
}

fn get_adjusted_mean(n: usize, k: usize, v: &[f64]) -> f64 {
    ((v[k] * k as f64) + v.iter().skip(k).take(n - k - k).sum::<f64>() + (v[n - k - 1] * k as f64))
        / n as f64
}
