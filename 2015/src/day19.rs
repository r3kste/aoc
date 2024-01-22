use std::collections::HashSet;

use itertools::enumerate;
use rand::seq::SliceRandom;

fn part1(input_file: &String) -> i32 {
    let mut ops: Vec<(&str, &str)> = Vec::new();
    let mut start = "";
    for line in input_file.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() > 1 {
            let fro = words[0];
            let to = words[2];
            ops.push((fro, to));
        } else if words.len() == 1 {
            start = words[0];
        }
    }
    let mut outcome: HashSet<String> = HashSet::new();
    for (from, towards) in ops {
        let parts: Vec<&str> = start.split(from).collect();
        if parts.len() == 1 {
            continue;
        }
        for i in 0..(parts.len() - 1) {
            let mut target = String::new();
            for (j, part) in enumerate(parts.clone()) {
                target.push_str(part);
                if j == i {
                    target.push_str(towards);
                } else {
                    if j != (parts.len() - 1) {
                        target.push_str(from);
                    }
                }
            }
            outcome.insert(target);
        }
    }

    return outcome.len().try_into().unwrap();
}

fn part2(input_file: &String) -> i32 {
    let mut ops: Vec<(&str, &str)> = Vec::new();
    let mut start = "";
    for line in input_file.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() > 1 {
            let fro = words[0];
            let to = words[2];
            ops.push((fro, to));
        } else if words.len() == 1 {
            start = words[0];
        }
    }
    let mut medicine = String::from(start);
    let mut steps = 0;
    while medicine != "e" {
        let temp = medicine.clone();
        for (from, towards) in ops.iter() {
            if medicine.find(towards) == None {
                continue;
            }
            let temp = medicine.clone();
            medicine = temp.replacen(towards, from, 1);
            steps += 1;
        }
        if temp == medicine {
            medicine = String::from(start);
            steps = 0;
            let mut rng = rand::thread_rng();
            ops.shuffle(&mut rng);
        }
    }
    return steps;
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day19.txt"),
    )
    .expect("File Not Found!");

    let _part1ans: i32 = part1(&input_file);
    println!("Day 19\nPart 1: {}", _part1ans);

    let _part2ans: i32 = part2(&input_file);
    println!("Part 2: {}", _part2ans);
}
