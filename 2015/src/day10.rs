fn part1n2(input_file: &String) -> (i32, i32) {
    let mut _part1ans: i32 = 0;
    let mut _part2ans: i32 = 0;
    let mut chars: Vec<char> = input_file.chars().collect();
    for op in 0..50 {
        let mut result: Vec<char> = Vec::new();
        let mut consecutive = 1;
        for i in 0..(chars.len() - 1) {
            if chars[i] == chars[i + 1] {
                consecutive += 1;
            } else {
                let temp: Vec<char> = consecutive.to_string().chars().collect();
                result.push(temp[0]);
                result.push(chars[i]);
                consecutive = 1;
            }
        }
        let temp: Vec<char> = consecutive.to_string().chars().collect();
        result.push(temp[0]);
        result.push(chars.last().unwrap().to_owned());
        chars = result;
        if op == 39 {
            _part1ans = chars.len().try_into().unwrap();
        }
        if op == 49 {
            _part2ans = chars.len().try_into().unwrap();
        }
    }
    return (_part1ans, _part2ans);
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day10.txt"),
    )
    .expect("File Not Found!");

    let (_part1ans, _part2ans): (i32, i32) = part1n2(&input_file);
    println!("Day 10\nPart 1: {}\nPart 2: {}", _part1ans, _part2ans);
}
