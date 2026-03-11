use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    // 0으로 시작되는 순열이 제거된 PERMUTATIONS
    const PERMUTATIONS: [usize; 10] =
        [1, 9, 81, 648, 4536, 27216, 136080, 544320, 1632960, 3265920];

    /*
    // 0으로 시작되는 순열이 제거된 PERMUTATIONS 구하기
    let factorial = |n| -> usize { (1..=n).product() };
    let permutation = |n, r| {
        if r > n {
            panic!("0 < r <= n 이여야 합니다. n: {n}, r: {r}");
        }

        factorial(n) / factorial(n - r)
    };
    for i in 0..10 {
        if i > 0 {
            PERMUTATIONS.push(permutation(10, i) - permutation(9, i - 1));
        } else {
            PERMUTATIONS.push(permutation(10, i));
        }
    }
    */

    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let lines = buf.lines().map(|x| x.parse::<usize>().unwrap());

    let mut out = String::new();
    for mut line in lines {
        if line == 0 {
            break;
        }

        line -= 1;
        let mut len = 1;
        while line >= PERMUTATIONS[len] {
            line -= PERMUTATIONS[len];
            len += 1;
        }

        let mut stack = Vec::new();

        for i in 0..len {
            let mut index = if i == 0 { 1 } else { 0 };
            for i in index..10 {
                if !stack.contains(&i) {
                    index = i;
                    break;
                }
            }
            let counts: usize = ((10 - (len - 1))..(10 - i)).product();

            let sub_count = line / counts;
            line %= counts;
            for _ in 0..sub_count {
                index += 1;
                for i in index..10 {
                    if !stack.contains(&i) {
                        index = i;
                        break;
                    }
                }
            }

            stack.push(index);
        }
        for i in stack {
            write!(out, "{i}").unwrap();
        }
        writeln!(out).unwrap();
    }

    print!("{out}");
}
