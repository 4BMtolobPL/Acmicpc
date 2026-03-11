use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace();
    let n = iter.next().unwrap().parse().unwrap();
    let v: Vec<i32> = iter.map(|x| x.parse().unwrap()).collect();

    let mut max = vec![0; n];

    // len = 2인 경우 -> 2장 모두 뽑은 경우만 가능
    if n > 1 {
        max[1] = get_bit_1(v[0] ^ v[1]);
    }

    // len = 3인 경우 -> 반드시 3장 모두 뽑은 경우만 가능
    if n > 2 {
        max[2] = get_bit_1(v[0] ^ v[1] ^ v[2]);
    }

    // len = 4인 경우 -> 반드시 2개 + 2개씩 뽑은 경우만 가능
    if n > 3 {
        max[3] = max[1] + get_bit_1(v[2] ^ v[3]);
    }

    for i in 4..n {
        max[i] = (max[i - 3] + get_bit_1(v[i - 2] ^ v[i - 1] ^ v[i]))
            .max(max[i - 2] + get_bit_1(v[i - 1] ^ v[i]));
    }

    println!("{}", max[n - 1]);
}

fn get_bit_1(n: i32) -> i32 {
    let mut count = 0;
    for i in 0..32 {
        if n & (1 << i) != 0 {
            count += 1;
        }
    }

    count
}
