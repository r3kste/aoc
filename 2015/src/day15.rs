use std::cmp::max;
use std::collections::HashMap;

fn part1n2(input_file: &HashMap<&str, Vec<i32>>) -> (i32, i32) {
    const SPOON: i32 = 100;
    let mut _part1ans = 0;
    let mut _part2ans = 0;

    for sprinkles in 0..=SPOON {
        for peanut in 0..=(SPOON - sprinkles) {
            for frost in 0..(SPOON - sprinkles - peanut) {
                let sugar = SPOON - sprinkles - peanut - frost;
                let mut score = 1;

                for property in 0..4 {
                    let total = max(
                        0,
                        sprinkles * (input_file["Sprinkles"][property])
                            + peanut * (input_file["PeanutButter"][property])
                            + frost * (input_file["Frosting"][property])
                            + sugar * (input_file["Sugar"][property]),
                    );
                    score *= total;
                    if score == 0 {
                        break;
                    }
                }
                if score == 0 {
                    continue;
                }

                let calorie = sprinkles * (input_file["Sprinkles"][4])
                    + peanut * (input_file["PeanutButter"][4])
                    + frost * (input_file["Frosting"][4])
                    + sugar * (input_file["Sugar"][4]);

                if calorie == 500 {
                    _part2ans = _part2ans.max(score);
                }
                _part1ans = _part1ans.max(score);
            }
        }
    }
    return (_part1ans, _part2ans);
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day15.txt"),
    )
    .expect("File Not Found!");

    let mut data: HashMap<&str, Vec<i32>> = HashMap::new();
    for line in input_file.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let ing = words[0].split(':').next().unwrap();
        let cap = words[2].split(',').next().unwrap().parse::<i32>().unwrap();
        let dur = words[4].split(',').next().unwrap().parse::<i32>().unwrap();
        let fla = words[6].split(',').next().unwrap().parse::<i32>().unwrap();
        let tex = words[8].split(',').next().unwrap().parse::<i32>().unwrap();
        let cal = words[10].split(',').next().unwrap().parse::<i32>().unwrap();
        data.insert(ing, vec![cap, dur, fla, tex, cal]);
    }

    let (_part1ans, _part2ans): (i32, i32) = part1n2(&data);
    println!("Day 15\nPart 1: {}\nPart 2: {}", _part1ans, _part2ans);
}
