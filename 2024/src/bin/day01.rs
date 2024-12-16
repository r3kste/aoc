use std::collections::HashMap;

fn part1(input_file: &String) -> i32 {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    for line in input_file.lines() {
        let nums: Vec<&str> = line.split("   ").collect();
        let (num1, num2): (i32, i32) = (nums[0].parse().unwrap(), nums[1].parse().unwrap());
        list1.push(num1);
        list2.push(num2);
    }
    list1.sort();
    list2.sort();

    let mut result = 0;
    for (n1, n2) in list1.iter().zip(list2.iter()) {
        result += (n1 - n2).abs();
    }

    return result;
}

fn part2(input_file: &String) -> i32 {
    let mut freq1: HashMap<i32, i32> = HashMap::new();
    let mut freq2: HashMap<i32, i32> = HashMap::new();
    for line in input_file.lines() {
        let nums: Vec<&str> = line.split("   ").collect();
        let (num1, num2): (i32, i32) = (nums[0].parse().unwrap(), nums[1].parse().unwrap());
        *freq1.entry(num1).or_insert(0) += 1;
        *freq2.entry(num2).or_insert(0) += 1;
    }

    let mut result = 0;
    for (k1, v1) in freq1.iter() {
        result += match freq2.get(k1) {
            Some(v2) => v2 * v1 * k1,
            None => 0,
        };
    }

    return result;
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join(".inputs")
            .join("day01.txt"),
    )
    .expect("File Not Found!");

    let _part1ans: i32 = part1(&input_file);
    println!("Day 1\nPart 1: {}", _part1ans);

    let _part2ans: i32 = part2(&input_file);
    println!("Part 2: {}", _part2ans);
}
