use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());

    let n: usize = iter.next().unwrap();
    let k = iter.next().unwrap();

    let mut v = vec![true; n + 1];
    v[0] = false;
    v[1] = false;

    let mut count = 0;
    'outer: for i in 2..=n {
        for j in (i..=n).step_by(i) {
            if v[j] {
                v[j] = false;
                count += 1;

                if count == k {
                    println!("{j}");
                    break 'outer;
                }
            }
        }
    }
}
