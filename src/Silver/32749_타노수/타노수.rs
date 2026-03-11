use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    let (n, t): (u32, u32) = {
        let line = lines.next().unwrap();
        let (x, y) = line.split_once(" ").unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    };
    let x = lines.next().unwrap();
    let slice_size = 2_usize.pow(n - t);

    let mut max: Option<&str> = None;
    for i in (0..x.len()).step_by(slice_size) {
        let slice = &x[i..(i + slice_size)];
        if let Some(s) = max {
            for j in 0..slice_size {
                if s.as_bytes()[j] > slice.as_bytes()[j] {
                    break;
                } else if s.as_bytes()[j] < slice.as_bytes()[j] {
                    max = Some(slice);
                } else {
                    continue;
                }
            }
        } else {
            max = Some(slice);
        }
    }

    println!("{}", max.unwrap());
}
