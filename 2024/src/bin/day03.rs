fn mul(instruction: &str) -> usize {
    let n = instruction.len();
    if &instruction[0..4] != "mul(" {
        return 0;
    }
    if &instruction[(n - 1)..] != ")" {
        return 0;
    }

    let instruction = &instruction[4..(n - 1)];
    let nums: Vec<usize> = instruction
        .split(",")
        .map(|num| match num.trim().parse::<_>() {
            Ok(n) => n,
            Err(_) => 0,
        })
        .collect();

    if nums.len() != 2 {
        return 0;
    }

    nums[0] * nums[1]
}

fn part1n2(input_file: &String) -> (i32, i32) {
    let mut enabled = true;
    let mut part1_result = 0;
    let mut part2_result = 0;
    for line in input_file.lines() {
        for idx in 0..line.len() - 3 {
            if &line[idx..(idx + 4)] == "mul(" {
                let closing_bracket = line[idx..].find(")").unwrap();
                part1_result += mul(&line[idx..(idx + closing_bracket + 1)]);
                if enabled {
                    part2_result += mul(&line[idx..(idx + closing_bracket + 1)]);
                }
            }
            if &line[idx..(idx + 2)] == "do" {
                if idx + 4 < line.len() && &line[idx..(idx + 4)] == "do()" {
                    enabled = true;
                } else if idx + 7 < line.len() && &line[idx..(idx + 7)] == "don't()" {
                    enabled = false;
                }
            }
        }
    }
    return (part1_result as i32, part2_result as i32);
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join(".inputs")
            .join("day03.txt"),
    )
    .expect("File Not Found!");

    // let _part1ans: i32 = part1n2(&input_file);
    let (_part1ans, _part2ans) = part1n2(&input_file);
    println!("Day 3\nPart 1: {}", _part1ans);
    println!("Part 2: {}", _part2ans);
}
