use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());

    let l: i32 = iter.next().unwrap();
    let r = iter.next().unwrap();
    let k = iter.next().unwrap();

    if k == 2 {
        // 2x + d = n
        let mut count = r - l + 1;
        if l < 3 {
            count -= 3 - l;
        }
        println!("{count}");
    } else if k == 4 {
        // (2x + 3d) * 2 = n
        let mut count = r / 2 - (l - 1).max(9) / 2;

        if (l..=r).contains(&12) {
            count -= 1;
        }

        if count < 0 {
            count = 0;
        }

        println!("{count}");
    } else {
        // k == 3일때
        // (x + d) * 3 = n
        // k == 5일때
        // (x + 2d) * 5 = i
        let mut count = r / k - (l - 1).max(k * k / 2) / k;
        if count < 0 {
            count = 0;
        }

        println!("{count}");
    }
}
