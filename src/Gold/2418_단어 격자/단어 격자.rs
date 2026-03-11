use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace();

    let h: usize = iter.next().unwrap().parse().unwrap();
    let w: usize = iter.next().unwrap().parse().unwrap();
    let _l: usize = iter.next().unwrap().parse().unwrap();

    let graph: Vec<Vec<_>> = iter.by_ref().take(h).map(|s| s.chars().collect()).collect();
    let word = iter.next().unwrap();

    let mut count = vec![vec![0_i64; w]; h];
    let mut visited = vec![vec![false; w]; h];

    let mut queue = VecDeque::new();
    let mut chars = word.chars();
    let first_char = chars.next().unwrap();
    for (r_index, row) in graph.iter().enumerate() {
        for (c_index, cell) in row.iter().enumerate() {
            if *cell == first_char {
                queue.push_back(Point::new(r_index, c_index));
                count[r_index][c_index] += 1;
            }
        }
    }

    for c in chars {
        let len = queue.len();

        let mut next_count = vec![vec![0_i64; w]; h];
        for _ in 0..len {
            let front = queue.pop_front().unwrap();

            for next_point in front.next_points(h, w) {
                if graph[next_point.x][next_point.y] == c {
                    if !visited[next_point.x][next_point.y] {
                        visited[next_point.x][next_point.y] = true;
                        queue.push_back(Point::new(next_point.x, next_point.y));
                    }

                    next_count[next_point.x][next_point.y] += count[front.x][front.y];
                }
            }
        }

        count = next_count;
        visited = vec![vec![false; w]; h];
    }

    println!("{}", count.iter().flatten().sum::<i64>());
}

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn next_points(&self, width: usize, height: usize) -> NextPoint {
        NextPoint {
            point: *self,
            width,
            height,
            index: 0,
        }
    }
}

struct NextPoint {
    point: Point,
    width: usize,
    height: usize,
    index: usize,
}

impl Iterator for NextPoint {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        const OFFSETS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        while let Some((dx, dy)) = OFFSETS.get(self.index) {
            self.index += 1;

            let next_x = self.point.x as isize + dx;
            let next_y = self.point.y as isize + dy;

            if (0_isize..self.width as isize).contains(&next_x)
                && (0_isize..self.height as isize).contains(&next_y)
            {
                return Some(Point {
                    x: next_x as usize,
                    y: next_y as usize,
                });
            }
        }

        None
    }
}
