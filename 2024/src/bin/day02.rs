fn is_safe(report: &Vec<i32>) -> bool {
    let is_increasing = report
        .windows(2)
        .all(|w| (w[1] - w[0] >= 1) && (w[1] - w[0] <= 3));
    let is_decreasing = report
        .windows(2)
        .all(|w| (w[0] - w[1] >= 1) && (w[0] - w[1] <= 3));
    return is_increasing || is_decreasing;
}

fn part1(input_file: &String) -> i32 {
    let mut result = 0;
    for line in input_file.lines() {
        let report: Vec<i32> = line
            .split(" ")
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();

        if is_safe(&report) {
            result += 1;
        }
    }

    return result;
}

fn part2(input_file: &String) -> i32 {
    let mut result = 0;
    for line in input_file.lines() {
        let report: Vec<i32> = line
            .split(" ")
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();

        if is_safe(&report) {
            result += 1;
        } else {
            for idx in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(idx);
                if is_safe(&new_report) {
                    result += 1;
                    break;
                }
            }
        }
    }
    return result;
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join(".inputs")
            .join("day02.txt"),
    )
    .expect("File Not Found!");

    let _part1ans: i32 = part1(&input_file);
    println!("Day 2\nPart 1: {}", _part1ans);

    let _part2ans: i32 = part2(&input_file);
    println!("Part 2: {}", _part2ans);
}
