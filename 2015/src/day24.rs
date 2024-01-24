use itertools::Itertools;

fn product(vec: &Vec<i32>) -> u64 {
    let mut prod: u64 = 1;
    for &i in vec {
        match prod.checked_mul(i as u64) {
            Some(val) => prod = val,
            None => return u64::MAX,
        }
    }
    prod
}

fn part1(weights: &Vec<i32>) -> u64 {
    let group_size = weights.iter().sum::<i32>() / 3;
    for i in 1..=weights.len() {
        let qes: Vec<u64> = weights
            .iter()
            .combinations(i)
            .filter_map(
                |c| match c.iter().map(|&&x| x).sum::<i32>().cmp(&group_size) {
                    std::cmp::Ordering::Equal => Some(product(&c.iter().map(|&&x| x).collect())),
                    _ => None,
                },
            )
            .collect();
        if !qes.is_empty() {
            return *qes.iter().min().unwrap();
        }
    }
    u64::MAX
}

fn part2(weights: &Vec<i32>) -> u64 {
    let group_size = weights.iter().sum::<i32>() / 4;
    for i in 1..=weights.len() {
        let qes: Vec<u64> = weights
            .iter()
            .combinations(i)
            .filter_map(
                |c| match c.iter().map(|&&x| x).sum::<i32>().cmp(&group_size) {
                    std::cmp::Ordering::Equal => Some(product(&c.iter().map(|&&x| x).collect())),
                    _ => None,
                },
            )
            .collect();
        if !qes.is_empty() {
            return *qes.iter().min().unwrap();
        }
    }
    u64::MAX
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day24.txt"),
    )
    .expect("File Not Found!");

    let weights: Vec<i32> = input_file
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let _part1ans: u64 = part1(&weights);
    println!("Day 24\nPart 1: {}", _part1ans);

    let _part2ans: u64 = part2(&weights);
    println!("Part 2: {}", _part2ans);
}
