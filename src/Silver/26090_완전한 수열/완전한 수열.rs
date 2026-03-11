use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let sequence: Vec<usize> = buf
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    // 소수 판정하기
    let max_prime_check = sequence.len() * 2000;
    let mut prime_numbers = vec![true; max_prime_check];
    prime_numbers[0] = false;
    prime_numbers[1] = false;

    for i in (2..).take_while(|x| x * x <= max_prime_check) {
        if prime_numbers[i] {
            for j in ((i * i)..max_prime_check).step_by(i) {
                prime_numbers[j] = false;
            }
        }
    }

    // 완전한 수열 구하기
    let mut count = 0;
    for len in 1..=sequence.len() {
        // 수열의 길이는 소수가 아닌경우 스킵
        if !prime_numbers[len] {
            continue;
        }

        for index in 0..sequence.len() {
            if let Some(slice) = sequence.get(index..(index + len)) {
                if prime_numbers[slice.iter().sum::<usize>()] {
                    count += 1;
                }
            }
        }
    }
    println!("{count}");
}
