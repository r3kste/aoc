use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

fn part1n2(input_file: &HashMap<&str, (i32, i32, i32)>) -> (i32, i32) {
    const TIME: i32 = 2503;
    let mut max_dist = 0;
    let mut point: HashMap<&str, i32> = HashMap::new();
    for time in 1..=TIME {
        let mut weindeer: Vec<&str> = Vec::new();
        for (_, (speed, travel, rest)) in input_file {
            let mut dist = 0;
            let cycle = travel + rest;
            dist += speed * travel * (time / (cycle));
            //    distance in 1 cycle  no of cycles
            dist += min((time % cycle) * speed, travel * speed);
            max_dist = max_dist.max(dist);
        }
        for (reindeer, (speed, travel, rest)) in input_file {
            let mut dist = 0;
            let cycle = travel + rest;
            dist += speed * travel * (time / (cycle));
            //    distance in 1 cycle  no of cycles
            dist += min((time % cycle) * speed, travel * speed);
            if dist == max_dist {
                weindeer.push(reindeer);
            }
        }
        for deer in weindeer {
            if !point.contains_key(deer) {
                point.insert(deer, 1);
            } else {
                *point.get_mut(deer).unwrap() += 1;
            }
        }
    }
    let mut max_point: i32 = 0;
    for (_, points) in point {
        max_point = max(max_point, points);
    }
    return (max_dist, max_point);
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day14.txt"),
    )
    .expect("File Not Found!");

    let mut data: HashMap<&str, (i32, i32, i32)> = HashMap::new();
    for line in input_file.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let u = words[0];
        let speed = words[3].parse::<i32>().unwrap();
        let travel = words[6].parse::<i32>().unwrap();
        let rest = words[13].parse::<i32>().unwrap();
        data.insert(u, (speed, travel, rest));
    }

    let (_part1ans, _part2ans): (i32, i32) = part1n2(&data);
    println!("Day 14\nPart 1: {}\nPart 2: {}", _part1ans, _part2ans);
}
