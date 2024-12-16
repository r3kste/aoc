use std::collections::HashMap;

fn is_valid(update: &Vec<i32>, rules: &HashMap<(i32, i32), i32>) -> bool {
    let n = update.len();

    for num1_idx in 0..n {
        let num1 = update[num1_idx];
        for num2_idx in (num1_idx + 1)..n {
            let num2 = update[num2_idx];
            match rules.get(&(num1, num2)) {
                Some(&val) => {
                    if val == 1 {
                        return false;
                    }
                }
                None => {}
            }
        }
    }
    true
}

fn part1(rules: &HashMap<(i32, i32), i32>, updates: &Vec<Vec<i32>>) -> i32 {
    updates
        .iter()
        .filter(|update| is_valid(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum::<i32>()
}

fn part2(rules: &HashMap<(i32, i32), i32>, updates: &Vec<Vec<i32>>) -> i32 {
    let compare = |a: &i32, b: &i32| {
        let (a, b) = (*a, *b);
        if !rules.contains_key(&(a, b)) {
            return std::cmp::Ordering::Equal;
        }
        match rules.get(&(a, b)) {
            Some(&val) => {
                if val == -1 {
                    return std::cmp::Ordering::Less;
                } else {
                    return std::cmp::Ordering::Greater;
                }
            }
            None => {
                // This should never happen
                return std::cmp::Ordering::Equal;
            }
        }
    };

    updates
        .iter()
        .filter(|update| !is_valid(update, &rules))
        .map(|update| {
            let mut new_update = update.clone();
            new_update.sort_by(compare);
            new_update[new_update.len() / 2]
        })
        .sum::<i32>()
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join(".inputs")
            .join("day05.txt"),
    )
    .expect("File Not Found!");

    let mut rules: HashMap<(i32, i32), i32> = HashMap::new();

    let mut idx = 0;
    let lines: Vec<&str> = input_file.lines().collect();
    while idx < lines.len() {
        let line = lines.get(idx).unwrap();
        idx += 1;
        if *line == "" {
            break;
        }

        let nums: Vec<i32> = line.split("|").map(|x| x.parse().unwrap()).collect();
        rules.insert((nums[0], nums[1]), -1);
        rules.insert((nums[1], nums[0]), 1);
    }

    let mut updates: Vec<Vec<i32>> = Vec::new();
    while idx < lines.len() {
        let line = lines.get(idx).unwrap();
        idx += 1;
        let update: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
        updates.push(update);
    }

    let _part1ans: i32 = part1(&rules, &updates);
    println!("Day 5\nPart 1: {}", _part1ans);

    let _part2ans: i32 = part2(&rules, &updates);
    println!("Part 2: {}", _part2ans);
}
