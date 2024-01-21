fn part1(input_file: &String) -> i32 {
    let vowels = "aeiou";
    let mut count = 0;
    for line in input_file.lines() {
        // println!("{}",line);
        let mut vowelcount = 0;
        let mut cosecutive_flag = false;
        let mut good = true;
        let mut prevchar = '.';
        for char in line.chars() {
            if vowels.contains(char) {
                vowelcount += 1;
            }
            if char == prevchar && !cosecutive_flag {
                cosecutive_flag = true;
            }
            prevchar = char;
        }
        let bad_strings = ["ab", "cd", "pq", "xy"];
        for bad_string in bad_strings.iter() {
            if line.contains(bad_string) {
                good = false;
                break;
            }
        }
        if good && (vowelcount >= 3) && cosecutive_flag {
            count += 1;
        }
    }
    return count;
}

fn part2(input_file: &String) -> i32 {
    let mut count = 0;
    for line in input_file.lines() {
        let mut repeat_pair = false;
        let mut repeat_gap = false;
        let n = line.len();
        'outer: for i in 0..n {
            for j in (i + 2)..(n - 1) {
                let first = &line[i..=(i + 1)];
                let second = &line[j..=(j + 1)];
                if first.eq(second) {
                    repeat_pair = true;
                    break 'outer;
                }
            }
        }
        for i in 0..(n - 2) {
            if (&line)[i..=i].eq(&line[(i + 2)..=(i + 2)]) {
                repeat_gap = true;
                break;
            }
        }
        if repeat_gap && repeat_pair {
            count += 1;
        }
    }
    return count;
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day05.txt"),
    )
    .expect("File Not Found!");

    let _part1ans: i32 = part1(&input_file);
    let _part2ans: i32 = part2(&input_file);

    println!("Day 05\nPart 1: {}\nPart 2: {}", _part1ans, _part2ans);
}
