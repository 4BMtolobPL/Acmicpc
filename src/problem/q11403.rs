use std::io::Write;
use std::io::{stdin, stdout, BufWriter, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let n = buf.lines().next().unwrap().parse().unwrap();

    let mut graph = Vec::with_capacity(n);

    for line in buf.lines().skip(1) {
        let l: Vec<usize> = line
            .split_whitespace()
            .map(|x| if x == "1" { 1 } else { 0 })
            .collect();
        graph.push(l);
    }

    for i in 0..n {
        for from in 0..n {
            for to in 0..n {
                if graph[from][i] == 0 || graph[i][to] == 0 {
                    continue;
                } else if graph[from][to] == 0 {
                    graph[from][to] = graph[from][i] + graph[i][to];
                } else {
                    graph[from][to] = graph[from][to].min(graph[from][i] + graph[i][to]);
                }
            }
        }
    }

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);

    for i in graph {
        let mut s = Vec::new();
        for j in i {
            match j {
                0 => s.push("0"),
                _ => s.push("1"),
            }
        }
        writeln!(out, "{}", s.join(" ")).unwrap();
    }
}
