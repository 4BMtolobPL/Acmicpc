use std::collections::{HashMap, HashSet};
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let lines = buf.lines();

    // name: 학생이름, 공백을 포함하지 않는 영어 소문자 10자 이하의 한 단어
    // place: 촬영장소, 공백을 포함하지 않는 영어 소문자 20자 이하의 한 단어
    // time: 촬영시간대, 공백으로 구분된 두 개의 시각, 시각은 5000 이하의 양의 정수, a <= x < b
    let mut names = HashSet::new();
    let mut places = HashMap::new();

    for line in lines.skip(1) {
        let mut iter = line.split_ascii_whitespace();
        let name = iter.next().unwrap().to_string();
        let place = iter.next().unwrap().to_string();
        let from: usize = iter.next().unwrap().parse().unwrap();
        let to: usize = iter.next().unwrap().parse().unwrap();

        if !names.contains(&name) {
            names.insert(name);
            places.entry(place.clone()).or_insert(vec![0; 50001]);

            if let Some(times) = places.get_mut(&place) {
                for val in times.iter_mut().take(to).skip(from) {
                    *val += 1;
                }
            }
        }
    }

    let mut max = 0;
    let mut max_v = Vec::new();

    for (place, times) in places.iter() {
        let mut lhs = None;
        for (time, count) in times.iter().enumerate() {
            if *count > max {
                max = *count;
                lhs = Some(time);
                max_v.clear();
            } else if *count == max {
                if lhs.is_none() {
                    lhs = Some(time);
                }
            } else if let Some(start) = lhs {
                max_v.push(PlaceTime::new(place.clone(), start, time));
                lhs = None;
            }
        }
    }

    max_v.sort_unstable_by(|a, b| a.place.cmp(&b.place).then(a.start.cmp(&b.start)));

    let first = max_v.first().unwrap();
    println!("{} {} {}", first.place, first.start, first.end);
}

struct PlaceTime {
    place: String,
    start: usize,
    end: usize,
}

impl PlaceTime {
    fn new(place: String, start: usize, end: usize) -> PlaceTime {
        PlaceTime { place, start, end }
    }
}
