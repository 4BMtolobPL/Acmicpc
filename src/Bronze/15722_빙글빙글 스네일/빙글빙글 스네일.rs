use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let time: i32 = buf.trim().parse().unwrap();

    let mut up = 0;
    let mut down = 0;
    let mut right = 0;
    let mut left = 0;

    let mut direction = 0;
    let mut l = 1;
    let mut count = 0;

    for _ in 0..time {
        match direction {
            0 => up += 1,
            1 => right += 1,
            2 => down += 1,
            3 => left += 1,
            _ => unreachable!(),
        }

        count += 1;

        if count == l {
            direction += 1;
            if direction > 3 {
                direction = 0;
            }

            if direction == 0 || direction == 2 {
                l += 1;
            }

            count = 0;
        }
    }

    println!("{} {}", right - left, up - down);
}
