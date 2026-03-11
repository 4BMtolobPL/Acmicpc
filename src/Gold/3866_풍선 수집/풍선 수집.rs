use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines();

    let mut out = String::new();
    loop {
        let n: usize = lines.next().unwrap().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            let mut iter = lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap());

            let position = iter.next().unwrap();
            let time = iter.next().unwrap();

            v.push(Balloon::new(position, time));
        }

        let result = get_distance(&v);

        if let Ok(x) = result {
            writeln!(out, "OK {x}").unwrap();
        } else {
            writeln!(out, "NG {}", result.unwrap_err()).unwrap();
        }
    }

    print!("{out}");
}

fn get_distance(balloons: &[Balloon]) -> Result<usize, usize> {
    // index: 운반중인 풍선 개수, Some((p, s)) p: 로봇 위치, s: 누적 거리
    let mut prev: Vec<Option<State>> = vec![None; 4];
    prev[0] = Some(State::new(0, 0));

    let mut prev_time = 0;
    for (ballon_num, next_balloon) in balloons.iter().enumerate() {
        let mut v: Vec<Option<State>> = vec![None; 4];
        for (i, state) in prev.iter().enumerate() {
            if let Some(prev_state) = state {
                // 바로 다음 위치로 가기
                if i < 3 {
                    let move_distance = prev_state.robot_position.abs_diff(next_balloon.position);
                    let move_time = move_distance * (i + 1);
                    // 시간 내로 이동할 수 있다면
                    if prev_time + move_time <= next_balloon.time {
                        if let Some(x) = v[i + 1] {
                            if prev_state.total_distance + move_distance < x.total_distance {
                                v[i + 1] = Some(State::new(
                                    next_balloon.position,
                                    prev_state.total_distance + move_distance,
                                ));
                            }
                        } else {
                            v[i + 1] = Some(State::new(
                                next_balloon.position,
                                prev_state.total_distance + move_distance,
                            ));
                        }
                    }
                }

                // 집 들렀다가 다음위치로 가기
                if i > 0 {
                    let move_distance = prev_state.robot_position + next_balloon.position;
                    let move_time = prev_state.robot_position * (i + 1) + next_balloon.position;
                    // 시간 내로 이동할 수 있다면
                    if prev_time + move_time <= next_balloon.time {
                        if let Some(x) = v[1] {
                            if prev_state.total_distance + move_distance < x.total_distance {
                                v[1] = Some(State::new(
                                    next_balloon.position,
                                    prev_state.total_distance + move_distance,
                                ));
                            }
                        } else {
                            v[1] = Some(State::new(
                                next_balloon.position,
                                prev_state.total_distance + move_distance,
                            ));
                        }
                    }
                }
            }
        }

        if v.iter().all(|x| x.is_none()) {
            return Err(ballon_num + 1);
        }

        prev = v;
        prev_time = next_balloon.time;
    }

    let mut min = usize::MAX;
    for val in prev.iter().skip(1).flatten() {
        min = min.min(val.robot_position + val.total_distance);
    }

    Ok(min)
}

struct Balloon {
    position: usize,
    time: usize,
}

impl Balloon {
    fn new(position: usize, time: usize) -> Balloon {
        Balloon { position, time }
    }
}

#[derive(Copy, Clone)]
struct State {
    robot_position: usize,
    total_distance: usize,
}

impl State {
    fn new(robot_position: usize, total_distance: usize) -> State {
        State {
            robot_position,
            total_distance,
        }
    }
}
