use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());

    let n: usize = iter.next().unwrap();
    let m = iter.next().unwrap();

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for _ in 0..m {
        let from = iter.next().unwrap();
        let to = iter.next().unwrap();

        graph[from].push(to);
        graph[to].push(from);
    }

    let dists_from_start = get_dists_paths(1, n, &graph);
    let dists_from_end = get_dists_paths(n, 1, &graph);

    // "도로를 통해 마을의 모든 장소를 오갈 수 있으며" 이므로 그냥 unwrap()
    let shortest_distance = dists_from_start[n].unwrap();

    // vertex_in_shortest: 최단거리에 포함되는 정점들
    let mut vertex_in_shortest = Vec::new();
    for i in 2..n {
        if let (Some(ds), Some(de)) = (dists_from_start[i], dists_from_end[i]) {
            if ds + de == shortest_distance {
                vertex_in_shortest.push(i);
            }
        }
    }

    let mut v = vec![Value::None; shortest_distance];
    for i in vertex_in_shortest.iter() {
        if let Some(d) = dists_from_start[*i] {
            match v[d] {
                Value::None => v[d] = Value::Singular(*i),
                Value::Singular(_) => v[d] = Value::Plural,
                Value::Plural => {}
            }
        }
    }

    if let Some(i) = v.iter().find_map(|x| x.get_singular()) {
        println!("{i}");
    } else {
        println!("1");
    }
}

#[derive(Clone)]
enum Value {
    None,
    Singular(usize),
    Plural,
}

impl Value {
    fn get_singular(&self) -> Option<usize> {
        if let Value::Singular(v) = self {
            Some(*v)
        } else {
            None
        }
    }
}

fn get_dists_paths(start: usize, end: usize, graph: &[Vec<usize>]) -> Vec<Option<usize>> {
    // None: 미방문 노드, Some(d) 해당 노드까지의 최단거리
    let mut dists = vec![None; graph.len()];
    dists[start] = Some(0);

    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut count = 1;
    'outer: while !queue.is_empty() {
        for _ in 0..queue.len() {
            let now = queue.pop_front().unwrap();

            for next in graph[now].iter() {
                if dists[*next].is_none() {
                    dists[*next] = Some(count);

                    // end까지의 최단거리를 구하였다면 더이상 각 노드들의 최단거리를 찾지 않고 종료
                    if *next == end {
                        break 'outer;
                    }
                    queue.push_back(*next);
                }
            }
        }

        count += 1;
    }

    dists
}
