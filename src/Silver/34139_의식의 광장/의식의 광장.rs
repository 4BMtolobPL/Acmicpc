use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines();

    // 1 <= _h <= 1000000000, 2 <= n <= 200000
    let (_h, n): (usize, usize) = {
        let mut iter = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    let mut points: Vec<Point> = vec![];
    for line in lines {
        let mut iter = line.split_ascii_whitespace().map(|x| x.parse().unwrap());
        let row_index = iter.next().unwrap();
        let col_index = iter.next().unwrap();

        points.push(Point::new(row_index, col_index));
    }

    let mut stack = vec![];
    let mut used = vec![false; n + 1];
    let mut set = HashSet::new();
    let mut map: HashMap<usize, Vec<MoveInfo>> = HashMap::new();

    let mut out = String::new();
    if dfs(n, &mut stack, &mut used, &mut set, &mut map, &points) {
        writeln!(out, "YES").unwrap();
        let first = stack.first().unwrap();
        write!(out, "{first}").unwrap();
        for i in stack.iter().skip(1) {
            write!(out, " {i}").unwrap();
        }
    } else {
        write!(out, "NO").unwrap();
    }

    println!("{out}");
}

fn dfs(
    n: usize,
    stack: &mut Vec<usize>,
    used: &mut Vec<bool>,
    set: &mut HashSet<usize>,
    map: &mut HashMap<usize, Vec<MoveInfo>>,
    points: &Vec<Point>,
) -> bool {
    if stack.len() == n {
        return true;
    }
    let point = points.get(stack.len()).unwrap();

    'outer: for i in 1..=n {
        if used[i] {
            continue 'outer;
        }

        let move_to = point.col + i;

        // 이동한 열에 이미 다른 빛이 있는 경우
        if set.contains(&(move_to)) {
            continue 'outer;
        }

        // 같은 행의 빛중 이미 이동한 빛과 겹치는 경우
        if map.contains_key(&point.row) {
            for prev_move in map.get(&point.row).unwrap() {
                if prev_move.from < point.col && prev_move.to > move_to
                    || prev_move.from > point.col && prev_move.to < move_to
                {
                    continue 'outer;
                }
            }
        }

        set.insert(move_to);
        (*map.entry(point.row).or_default()).push(MoveInfo::new(point.col, move_to));
        stack.push(i);
        used[i] = true;
        if dfs(n, stack, used, set, map, points) {
            return true;
        }
        used[i] = false;
        stack.pop();
        (*map.get_mut(&point.row).unwrap()).pop();
        set.remove(&move_to);
    }

    false
}

struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

struct MoveInfo {
    from: usize,
    to: usize,
}

impl MoveInfo {
    fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }
}
