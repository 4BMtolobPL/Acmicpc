use std::cmp::Reverse;
use std::io::Write;
use std::{
    collections::{BinaryHeap, HashMap},
    io::{stdin, stdout, BufWriter, Read},
    str::Lines,
};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    let test_case = lines.next().unwrap().parse().unwrap();
    for _ in 0..test_case {
        cycle(&mut lines);
    }
}

fn cycle(lines: &mut Lines) {
    let k = lines.next().unwrap().parse().unwrap();

    let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    let mut deleted_max = HashMap::new();
    let mut deleted_min = HashMap::new();
    for _ in 0..k {
        let (command, value) = lines.next().unwrap().trim().split_once(' ').unwrap();
        match command {
            "I" => {
                let value = value.parse().unwrap();
                max_heap.push(value);
                min_heap.push(Reverse(value));
            }
            "D" => {
                if max_heap.is_empty() {
                    continue;
                }
                match value {
                    "1" => {
                        let counter = deleted_max.entry(max_heap.pop().unwrap()).or_insert(0);
                        *counter += 1;
                    }
                    "-1" => {
                        let counter = deleted_min.entry(min_heap.pop().unwrap().0).or_insert(0);
                        *counter += 1;
                    }
                    _ => {}
                }
                trim_max_heap(&mut max_heap, &mut deleted_min);
                trim_min_heap(&mut min_heap, &mut deleted_max);
            }
            _ => {}
        }
    }

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);
    if max_heap.is_empty() {
        writeln!(out, "EMPTY").unwrap();
    } else {
        writeln!(
            out,
            "{} {}",
            max_heap.peek().unwrap(),
            min_heap.peek().unwrap().0
        )
        .unwrap();
    }
}

fn trim_max_heap(heap: &mut BinaryHeap<i32>, deleted_map: &mut HashMap<i32, i32>) {
    if let Some(x) = heap.peek() {
        let mut max = *x;
        while deleted_map.contains_key(&max) {
            for _ in 0..*deleted_map.get(&max).unwrap() {
                heap.pop();
            }
            deleted_map.remove(&max);
            if let Some(x) = heap.peek() {
                max = *x;
            }
        }
    }
}

fn trim_min_heap(heap: &mut BinaryHeap<Reverse<i32>>, deleted_map: &mut HashMap<i32, i32>) {
    if let Some(x) = heap.peek() {
        let mut min = x.0;
        while deleted_map.contains_key(&min) {
            for _ in 0..*deleted_map.get(&min).unwrap() {
                heap.pop();
            }
            deleted_map.remove(&min);
            if let Some(x) = heap.peek() {
                min = x.0;
            }
        }
    }
}
