use std::io::prelude::*;
// use std::{thread, time};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let lines: Vec<&str> = buf.split_whitespace().collect();
    let n: u64 = lines[1].parse().unwrap();
    let mut nums: Vec<u64> = Vec::new();
    for line in lines.iter().skip(2) {
        nums.push(line.parse().unwrap());
    }
    
    let sum: u64 = nums.iter().sum();
    let mut lhs: u64 = 1;
    let mut rhs: u64 = sum / n;
    while lhs <= rhs {
        let middle = (rhs + lhs) / 2;
        
        let mut count = 0;
        for num in &nums {
            count += num / middle;
        }

        if count >= n {
            lhs = middle + 1;
        } else if count < n {
            rhs = middle - 1;
        }

        // println!("lhs: {}, rhs: {}", lhs, rhs);
        // thread::sleep(time::Duration::from_millis(500));
    }
    println!("{}", rhs);
}