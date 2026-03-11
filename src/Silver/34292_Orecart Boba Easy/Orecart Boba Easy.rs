use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let l: i32 = iter.next().unwrap().parse().unwrap();

    let boba_stops: Vec<i32> = iter.by_ref().take(n).map(|x| x.parse().unwrap()).collect();
    let waiting_times: Vec<i32> = iter.by_ref().take(n).map(|x| x.parse().unwrap()).collect();
    let v: i32 = iter.next().unwrap().parse().unwrap();

    let mut yk_position = 0;
    let mut waiting = 0;
    for (stop_position, waiting_time) in boba_stops.iter().zip(waiting_times.iter()) {
        // 유진캘리가 이동해야하는 거리
        let yk_distance = stop_position - yk_position;
        // 유진캘리에게 주어진 이동시간
        let yk_time = yk_distance - waiting;

        if yk_time * v >= yk_distance {
            // 따라잡을 수 있다면
            // 카트와 유진캘리를 해당 보바스탑으로 이동하고, waiting = waiting_time
            yk_position = *stop_position;
            waiting = *waiting_time;
        } else {
            // 따라잡을 수 없다면
            // 유진캘리는 정지, waiting += waiting_time
            waiting += waiting_time;
        }
    }

    // 골인지점까지 유진캘리가 이동해야하는 거리
    let yk_distance = l - yk_position;
    // 유진캘리에게 주어진 이동시간
    let yk_time = yk_distance - waiting;

    if yk_time * v >= yk_distance {
        println!("YES");
    } else {
        println!("NO");
    }
}
