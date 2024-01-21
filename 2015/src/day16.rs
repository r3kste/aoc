use std::collections::HashMap;

fn part1(input_file: &String) -> i32 {
    let known: HashMap<&str, i32> = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .into();
    for line in input_file.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let sue = words[1][0..words[1].len() - 1].parse::<i32>().unwrap();
        let info1 = (
            &words[2][0..words[2].len() - 1],
            words[3][0..words[3].len() - 1].parse::<i32>().unwrap(),
        );
        let info2 = (
            &words[4][0..words[4].len() - 1],
            words[5][0..words[5].len() - 1].parse::<i32>().unwrap(),
        );
        let info3 = (
            &words[6][0..words[6].len() - 1],
            words[7][0..words[7].len()].parse::<i32>().unwrap(),
        );

        let mut possible = true;
        for (property, quantity) in [info1, info2, info3] {
            if known[property] != quantity {
                possible = false;
                break;
            }
        }
        if possible {
            return sue;
        }
    }
    return 0;
}

fn part2(input_file: &String) -> i32 {
    let known: HashMap<&str, i32> = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .into();
    for line in input_file.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let sue = words[1][0..words[1].len() - 1].parse::<i32>().unwrap();
        let info1 = (
            &words[2][0..words[2].len() - 1],
            words[3][0..words[3].len() - 1].parse::<i32>().unwrap(),
        );
        let info2 = (
            &words[4][0..words[4].len() - 1],
            words[5][0..words[5].len() - 1].parse::<i32>().unwrap(),
        );
        let info3 = (
            &words[6][0..words[6].len() - 1],
            words[7][0..words[7].len()].parse::<i32>().unwrap(),
        );

        let mut possible = true;
        for (property, quantity) in [info1, info2, info3] {
            if property == "cats" || property == "trees" {
                if known[property] >= quantity {
                    possible = false;
                    break;
                }
            } else if property == "pomerians" || property == "goldfish" {
                if known[property] <= quantity {
                    possible = false;
                    break;
                }
            } else if known[property] != quantity {
                possible = false;
                break;
            }
        }
        if possible {
            return sue;
        }
    }
    return 0;
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day16.txt"),
    )
    .expect("File Not Found!");

    let _part1ans: i32 = part1(&input_file);
    println!("Day 16\nPart 1: {}", _part1ans);

    let _part2ans: i32 = part2(&input_file);
    println!("Part 2: {}", _part2ans);
}
