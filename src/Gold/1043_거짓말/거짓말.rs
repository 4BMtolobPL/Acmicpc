use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    
    let m: usize = lines
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();

    let mut truthers: BTreeSet<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let parties: Vec<BTreeSet<i32>> = lines
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();
    let mut human_party_map: BTreeMap<i32, Vec<_>> = BTreeMap::new();

    for (party_index, party) in parties.iter().enumerate() {
        for i in party.iter() {
            human_party_map
                .entry(*i)
                .or_insert_with(Vec::new)
                .push(party_index);
        }
    }

    let mut stack: Vec<i32> = Vec::new();
    let mut result: Vec<bool> = vec![true; m];
    for i in truthers.iter() {
        stack.push(*i);
    }
    while let Some(truther) = stack.pop() {
        let parties_of_truther = human_party_map.get(&truther);
        if let Some(party_of_truther) = parties_of_truther {
            for party_index in party_of_truther {
                result[*party_index] = false;
                for j in parties[*party_index].iter() {
                    if !truthers.contains(j) {
                        truthers.insert(*j);
                        stack.push(*j);
                    }
                }
            }
        }
    }

    println!("{}", result.iter().filter(|x| **x).count());
}
