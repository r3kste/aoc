use std::collections::HashMap;

use itertools::Itertools;

fn part1n2(input_file: &HashMap<&str, HashMap<&str, i32>>) -> (i32, i32) {
    let mut cities: Vec<&str> = input_file.keys().cloned().collect();
    cities.sort();
    let n = cities.len();
    let permutations = (0..n).into_iter().permutations(n);
    let mut min_dist = i32::MAX;
    let mut max_dist = 0;
    for permutation in permutations {
        let mut dist = 0;
        for i in 0..(n - 1) {
            dist += input_file[cities[permutation[i + 1]]][cities[permutation[i]]];
        }
        min_dist = min_dist.min(dist);
        max_dist = max_dist.max(dist);
    }
    return (min_dist, max_dist);
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day09.txt"),
    )
    .expect("File Not Found!");

    let mut data: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    for line in input_file.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let u = words[0];
        let v = words[2];
        let d = words[4].parse::<i32>().unwrap();

        if !data.contains_key(u) {
            data.insert(u, HashMap::new());
        }
        if !data.contains_key(v) {
            data.insert(v, HashMap::new());
        }
        data.get_mut(u).unwrap().insert(v, d);
        data.get_mut(v).unwrap().insert(u, d);
    }

    let (_part1ans, _part2ans): (i32, i32) = part1n2(&data);
    println!("Day 09\nPart 1: {}\nPart 2: {}", _part1ans, _part2ans);
}
