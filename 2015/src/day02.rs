use std::cmp::min;

fn part1(input_file: &String) -> i32 {
    let mut sum: i32 = 0;
    for line in input_file.lines() {
        let parts: Vec<i32> = line.split('x').map(|part| part.parse().unwrap()).collect();
        let (a, b, c) = (parts[0], parts[1], parts[2]);
        sum += 2 * (a * b + b * c + c * a);
        sum += min(a * b, min(b * c, c * a));
    }
    return sum;
}

fn part2(input_file: &String) -> i32 {
    let mut sum: i32 = 0;
    for line in input_file.lines() {
        let parts: Vec<i32> = line.split('x').map(|part| part.parse().unwrap()).collect();
        let (a, b, c) = (parts[0], parts[1], parts[2]);
        sum += 2 * min(a + b, min(b + c, c + a));
        sum += a * b * c;
    }
    return sum;
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day02.txt"),
    )
    .expect("File Not Found!");

    let _part1ans: i32 = part1(&input_file);
    let _part2ans: i32 = part2(&input_file);

    println!("Day 02\nPart 1: {}\nPart 2: {}", _part1ans, _part2ans);
}
