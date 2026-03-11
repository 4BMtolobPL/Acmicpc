use std::io;
use std::io::stdin;

fn main() {
    let buf = io::read_to_string(stdin()).unwrap();

    let mut iter = buf.split_ascii_whitespace();
    let n: u64 = iter.next().unwrap().parse().unwrap();
    let m: u64 = iter.next().unwrap().parse().unwrap();
    let trees: Vec<u64> = iter.map(|x| x.parse().unwrap()).collect();

    let min = *trees.iter().min().unwrap();
    let min_sum: u64 = trees.iter().map(|x| (*x).saturating_sub(min)).sum();

    if min_sum > m {
        let mut lhs = min;
        let mut rhs = *trees.iter().max().unwrap();

        while lhs <= rhs {
            let center = (lhs + rhs) / 2;
            let sum: u64 = trees.iter().map(|x| (*x).saturating_sub(center)).sum();
            if sum < m {
                rhs = center - 1;
            } else if sum > m {
                lhs = center + 1;
            } else {
                break;
            }
        }

        println!("{}", (lhs + rhs) / 2);
    } else {
        let x = if (m - min_sum) % n == 0 {
            (m - min_sum) / n
        } else {
            (m - min_sum) / n + 1
        };
        println!("{}", min - x)
    }
}
