fn part1(input_file: &String) -> i32 {
    let mut result = 0;
    return result;
}

fn part2(input_file: &String) -> i32 {
    let mut result = 0;
    return result;
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join(".inputs")
            .join("day05.txt"),
    )
    .expect("File Not Found!");

    let start = std::time::Instant::now();
    let _part1ans = part1(&input_file);
    let duration = start.elapsed();
    println!("Day 7\nPart 1: {}\t(took {:?})", _part1ans, duration);

    let start = std::time::Instant::now();
    let _part2ans = part2(&input_file);
    let duration = start.elapsed();
    println!("Part 2: {}\t(took {:?})", _part2ans, duration);
}
