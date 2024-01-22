fn part1(input_file: &Vec<(String, char, i32)>) -> i32 {
    let mut ptr: i32 = 0;
    let mut a = 0;
    let mut b = 0;
    let n: i32 = input_file.len() as i32;
    while ptr < n && ptr >= 0 {
        let (typ, target, value) = input_file[ptr as usize].clone();
        if typ == "hlf" {
            if target == 'a' {
                a /= 2;
            } else {
                b /= 2;
            }
            ptr += 1;
        } else if typ == "tpl" {
            if target == 'a' {
                a *= 3;
            } else {
                b *= 3;
            }
            ptr += 1;
        } else if typ == "inc" {
            if target == 'a' {
                a += 1;
            } else {
                b += 1;
            }
            ptr += 1;
        } else if typ == "jmp" {
            ptr = ptr + value;
        } else if typ == "jie" {
            if target == 'a' {
                if a % 2 == 0 {
                    ptr = ptr + value;
                } else {
                    ptr += 1;
                }
            } else {
                if b % 2 == 0 {
                    ptr = ptr + value;
                } else {
                    ptr += 1;
                }
            }
        } else if typ == "jio" {
            if target == 'a' {
                if a == 1 {
                    ptr = ptr + value;
                } else {
                    ptr += 1;
                }
            } else {
                if b == 1 {
                    ptr = ptr + value;
                } else {
                    ptr += 1;
                }
            }
        }
    }
    return b;
}

fn part2(input_file: &Vec<(String, char, i32)>) -> i32 {
    let mut ptr = 0;
    let mut a = 1;
    let mut b = 0;
    let n: i32 = input_file.len() as i32;
    while ptr < n && ptr >= 0 {
        let (typ, target, value) = input_file[ptr as usize].clone();
        if typ == "hlf" {
            if target == 'a' {
                a /= 2;
            } else {
                b /= 2;
            }
            ptr += 1;
        } else if typ == "tpl" {
            if target == 'a' {
                a *= 3;
            } else {
                b *= 3;
            }
            ptr += 1;
        } else if typ == "inc" {
            if target == 'a' {
                a += 1;
            } else {
                b += 1;
            }
            ptr += 1;
        } else if typ == "jmp" {
            ptr = ptr + value;
        } else if typ == "jie" {
            if target == 'a' {
                if a % 2 == 0 {
                    ptr = ptr + value;
                } else {
                    ptr += 1;
                }
            } else {
                if b % 2 == 0 {
                    ptr = ptr + value;
                } else {
                    ptr += 1;
                }
            }
        } else if typ == "jio" {
            if target == 'a' {
                if a == 1 {
                    ptr = ptr + value;
                } else {
                    ptr += 1;
                }
            } else {
                if b == 1 {
                    ptr = ptr + value;
                } else {
                    ptr += 1;
                }
            }
        }
    }
    return b;
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day23.txt"),
    )
    .expect("File Not Found!");

    let mut instructions: Vec<(String, char, i32)> = Vec::new();

    for line in input_file.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let typ = String::from(words[0]);
        let target = words[1].chars().nth(0).unwrap();
        let value;
        if typ == "inc" {
            value = 1;
        } else if typ == "hlf" {
            value = 2;
        } else if typ == "tpl" {
            value = 3;
        } else if typ == "jmp" {
            value = words[1].parse::<i32>().unwrap();
        } else if typ == "jie" {
            value = words[2].parse::<i32>().unwrap();
        } else if typ == "jio" {
            value = words[2].parse::<i32>().unwrap();
        } else {
            value = 0;
        }
        instructions.push((typ, target, value));
    }

    let _part1ans: i32 = part1(&instructions);
    println!("Day 23\nPart 1: {}", _part1ans);

    let _part2ans: i32 = part2(&instructions);
    println!("Part 2: {}", _part2ans);
}
