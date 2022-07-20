use std::collections::HashMap;
use std::io::{self, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let testcase: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..testcase {
        let targetline = lines.next().unwrap();
        let target: usize = targetline.split_whitespace().skip(1).next().unwrap().parse().unwrap();

        let queue: Vec<i32> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        println!("{}", func(target, &queue));
    }




}

fn func(target: usize, queue: &Vec<i32>) -> i32 {
    let mut hashmap = HashMap::new();
    let mut printed = vec![false; queue.len()];
    for value in queue {
        let x = hashmap.entry(*value).or_insert(0);
        *x += 1;
    }
    let mut keys: Vec<&i32> = hashmap.keys().map(|x| x).collect();
    keys.sort_by(|a, b| b.cmp(a));

    let mut queue_iter = queue.iter().enumerate().cycle();

    let mut counter = 0;

    for key in keys {
        let mut left = *hashmap.get(key).unwrap();
        while left > 0 {
            let (index, value) = queue_iter.next().unwrap();
            if printed[index] {
                continue;
            }
            if value == key {
                printed[index] = true;
                counter += 1;
                left -= 1;
                if index == target {
                    return counter;
                }
            }
        }
    }
    -1
}