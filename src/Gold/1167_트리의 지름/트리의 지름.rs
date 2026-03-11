use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace();

    let v: usize = iter.next().unwrap().parse().unwrap();
    let mut tree = vec![vec![]; v + 1];

    for _ in 0..v {
        let index: usize = iter.next().unwrap().parse().unwrap();

        loop {
            let next = iter.next().unwrap();
            if next == "-1" {
                break;
            }
            let next_index: usize = next.parse().unwrap();
            let cost = iter.next().unwrap().parse().unwrap();

            tree[index].push(Edge::new(next_index, cost));
        }
    }

    println!("{}", get_diameter(v, &tree));
}

fn get_diameter(v: usize, tree: &[Vec<Edge>]) -> i32 {
    let mut distance = vec![None; v + 1];
    distance[1] = Some(0);

    dfs(1, &mut distance, tree);

    let max_index = distance
        .iter()
        .enumerate()
        .filter(|(_, val)| val.is_some())
        .max_by_key(|(_, val)| val.unwrap())
        .unwrap()
        .0;
    for i in distance.iter_mut() {
        *i = None;
    }
    distance[max_index] = Some(0);

    dfs(max_index, &mut distance, tree);

    distance.iter().filter_map(|val| *val).max().unwrap()
}

fn dfs(index: usize, distance: &mut Vec<Option<i32>>, tree: &[Vec<Edge>]) {
    for next in tree[index].iter() {
        if distance[next.next].is_none() {
            distance[next.next] = Some(distance[index].unwrap() + next.cost);
            dfs(next.next, distance, tree);
        }
    }
}

#[derive(Clone)]
struct Edge {
    next: usize,
    cost: i32,
}

impl Edge {
    fn new(next: usize, cost: i32) -> Self {
        Self { next, cost }
    }
}
