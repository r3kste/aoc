use std::collections::HashMap;

use itertools::Itertools;

fn part1(input_file: &HashMap<&str, HashMap<&str, i32>>) -> i32 {
    let mut peeps: Vec<&str> = input_file.keys().cloned().collect();
    peeps.sort();
    let n = peeps.len();
    let permutations = (0..n).into_iter().permutations(n);
    let mut happiness = 0;
    for permutation in permutations {
        let mut hap = 0;
        for i in 0..(n - 1) {
            hap += input_file[peeps[permutation[i]]][peeps[permutation[i + 1]]]
                + input_file[peeps[permutation[i + 1]]][peeps[permutation[i]]];
        }
        hap += input_file[peeps[permutation[n - 1]]][peeps[permutation[0]]]
            + input_file[peeps[permutation[0]]][peeps[permutation[n - 1]]];
        happiness = happiness.max(hap);
    }
    return happiness;
}

fn part2(input_file: &HashMap<&str, HashMap<&str, i32>>) -> i32 {
    let mut peeps: Vec<&str> = input_file.keys().cloned().collect();

    let mut input_file = input_file.to_owned();
    input_file.insert("You", HashMap::new());
    for peep in peeps.iter() {
        input_file.get_mut(peep).unwrap().insert("You", 0);
        input_file.get_mut("You").unwrap().insert(peep, 0);
    }

    peeps.insert(0, "You");
    peeps.sort();
    let n = peeps.len();

    let permutations = (0..n).into_iter().permutations(n);
    let mut happiness = 0;
    for permutation in permutations {
        let mut hap = 0;
        for i in 0..(n - 1) {
            hap += input_file[peeps[permutation[i]]][peeps[permutation[i + 1]]]
                + input_file[peeps[permutation[i + 1]]][peeps[permutation[i]]];
        }
        hap += input_file[peeps[permutation[n - 1]]][peeps[permutation[0]]]
            + input_file[peeps[permutation[0]]][peeps[permutation[n - 1]]];
        happiness = happiness.max(hap);
    }
    return happiness;
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day13.txt"),
    )
    .expect("File Not Found!");

    let mut data: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    for line in input_file.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let u = words[0];
        let v = words[10].split('.').next().unwrap();
        let mut d = words[3].parse::<i32>().unwrap();
        let sign = words[2];

        if sign == "lose" {
            d *= -1;
        }

        if !data.contains_key(u) {
            data.insert(u, HashMap::new());
        }
        data.get_mut(u).unwrap().insert(v, d);
    }

    let _part1ans: i32 = part1(&data);
    println!("Day 13\nPart 1: {}", _part1ans);

    let _part2ans: i32 = part2(&data);
    println!("Part 2: {}", _part2ans);
}
