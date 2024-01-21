fn part1(input_file: &String) -> i32 {
    let mut number = 1;
    loop {
        let md5 = md5::compute(format!("{}{}", input_file, number));
        let hash = format!("{:x}", md5);
        if &hash[..5] == "00000" {
            break;
        }
        number += 1;
    }
    return number;
}

fn part2(input_file: &String) -> i32 {
    let mut number = part1(&input_file);
    loop {
        let md5 = md5::compute(format!("{}{}", input_file, number));
        let hash = format!("{:x}", md5);
        if &hash[..6] == "000000" {
            break;
        }
        number += 1;
    }
    return number;
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day04.txt"),
    )
    .expect("File Not Found!");

    println!("Takes quite a bit of time (around 20 seconds)");

    let _part1ans: i32 = part1(&input_file);
    let _part2ans: i32 = part2(&input_file);

    println!("Day 04 \nPart 1: {}\nPart 2: {}", _part1ans, _part2ans);
}
