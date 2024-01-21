fn part1(input_file: &String) -> i32 {
    let mut count = 0;
    for line in input_file.lines() {
        let mut diff = 2;
        let mut i: usize = 1;
        while i < line.len() - 1 {
            let first = line.chars().nth(i).unwrap();
            if first == '\\' {
                if line.chars().nth(i + 1).unwrap() == 'x' {
                    diff += 3;
                    i += 4;
                } else {
                    i += 2;
                    diff += 1;
                }
            } else {
                i += 1;
            }
        }
        count += diff;
    }
    return count;
}

fn part2(input_file: &String) -> i32 {
    let mut count = 0;
    for line in input_file.lines() {
        let expanded = line.escape_default().to_string();
        count += expanded.len() - line.len() + 2;
    }
    return count.try_into().unwrap();
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day08.txt"),
    )
    .expect("File Not Found!");

    let _part1ans: i32 = part1(&input_file);
    let _part2ans: i32 = part2(&input_file);

    println!("Day 08\nPart 1: {}\nPart 2: {}", _part1ans, _part2ans);
}
