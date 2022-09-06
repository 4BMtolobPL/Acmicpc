use std::io::{stdin, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let n: usize = lines
        .next()
        .unwrap()
        .split_whitespace()
        .take(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();

    // 자기 자신으로 가는 비용 0으로 초기화
    let mut graph = vec![vec![None; n + 1]; n + 1];
    for (index_from, iner) in graph.iter_mut().enumerate() {
        for (index_to, value) in iner.iter_mut().enumerate() {
            if index_from == index_to {
                *value = Some(0);
            }
        }
    }

    // 입력한 연결된 간선 양방향 초기화
    for line in lines {
        let mut iter = line.split_whitespace().map(|x| x.parse().unwrap());
        let a: usize = iter.next().unwrap();
        let b: usize = iter.next().unwrap();
        graph[a][b] = Some(1);
        graph[b][a] = Some(1);
    }

    // 플로이드-워셜
    for i in 1..=n {
        for j in 1..=n {
            for k in 1..=n {
                graph[j][k] = if graph[j][i] == None || graph[i][k] == None {
                    graph[j][k]
                } else if graph[j][k] == None {
                    Some(graph[j][i].unwrap() + graph[i][k].unwrap())
                } else {
                    Some(
                        graph[j][k]
                            .unwrap()
                            .min(graph[j][i].unwrap() + graph[i][k].unwrap()),
                    )
                };
            }
        }
    }

    let mut bacon = Vec::new();
    for i in graph.iter().skip(1) {
        let mut sum = 0;
        for value in i.iter().flatten() {
            sum += value;
        }
        bacon.push(sum);
    }

    let (mut min, mut min_index) = (i32::MAX, 0);
    for (index, value) in bacon.iter().enumerate() {
        if *value < min {
            min_index = index;
            min = *value;
        }
    }

    println!("{}", min_index + 1);
}
