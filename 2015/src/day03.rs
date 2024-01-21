use std::collections::{HashMap, HashSet};

fn add_tuples(a: &(i32, i32), b: &(i32, i32)) -> (i32, i32) {
    return (a.0 + b.0, a.1 + b.1);
}

fn part1(input_file: &String) -> i32 {
    let dir: HashMap<char, (i32, i32)> =
        [('^', (0, 1)), ('v', (0, -1)), ('<', (-1, 0)), ('>', (1, 0))]
            .iter()
            .cloned()
            .collect();
    let mut pos = (0, 0);
    let mut travelled: HashSet<(i32, i32)> = HashSet::new();
    travelled.insert(pos);
    for char in input_file.chars() {
        pos = add_tuples(&pos, &dir.get(&char).unwrap());
        travelled.insert(pos);
    }
    return travelled.len().try_into().unwrap();
}

fn part2(input_file: &String) -> i32 {
    let dir: HashMap<char, (i32, i32)> =
        [('^', (0, 1)), ('v', (0, -1)), ('<', (-1, 0)), ('>', (1, 0))]
            .iter()
            .cloned()
            .collect();

    let mut santa_pos = (0, 0);
    let mut robot_pos = (0, 0);
    let mut travelled = HashSet::new();
    travelled.insert(santa_pos);
    let mut instruction_count: i32 = 0;

    for char in input_file.chars() {
        if instruction_count % 2 == 0 {
            santa_pos = add_tuples(&santa_pos, &dir.get(&char).unwrap());
            travelled.insert(santa_pos);
        } else {
            robot_pos = add_tuples(&robot_pos, &dir.get(&char).unwrap());
            travelled.insert(robot_pos);
        }
        instruction_count += 1;
    }
    return travelled.len().try_into().unwrap();
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day03.txt"),
    )
    .expect("File Not Found!");

    let _part1ans: i32 = part1(&input_file);
    let _part2ans: i32 = part2(&input_file);

    println!("Day 03\nPart 1: {}\nPart 2: {}", _part1ans, _part2ans);
}
