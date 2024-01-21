fn combinations(
    n: i32,
    i: usize,
    containers: &Vec<i32>,
    mut picked: Vec<i32>,
    mut combination: &mut Vec<Vec<i32>>,
) -> i32 {
    if n < 0 {
        return 0;
    } else if n == 0 {
        combination.push(picked);
        return 1;
    } else if i == containers.len() {
        return 0;
    } else {
        let unpick = combinations(n, i + 1, &containers, picked.clone(), &mut combination);
        picked.push(containers[i]);
        let pick = combinations(
            n - containers[i],
            i + 1,
            &containers,
            picked,
            &mut combination,
        );
        return pick + unpick;
    }
}

fn part1n2(input_file: &String) -> (i32, i32) {
    const TOTAL: i32 = 150;
    let mut _part1ans = 0;
    let mut _part2ans = 0;
    let mut containers: Vec<i32> = Vec::new();
    for line in input_file.lines() {
        let num = line.parse::<i32>().unwrap();
        containers.push(num);
    }
    containers.sort();

    let picked: Vec<i32> = Vec::new();
    let mut combination: Vec<Vec<i32>> = Vec::new();
    _part1ans = combinations(TOTAL, 0, &containers, picked, &mut combination);

    let mut min_len = 1000;
    for comb in combination.clone() {
        min_len = std::cmp::min(min_len, comb.len());
    }
    let mut count = 0;
    for comb in combination {
        if comb.len() == min_len {
            count += 1;
        }
    }
    _part2ans = count;
    return (_part1ans, _part2ans);
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day17.txt"),
    )
    .expect("File Not Found!");

    let (_part1ans, _part2ans): (i32, i32) = part1n2(&input_file);
    println!("Day 17\nPart 1: {}\nPart 2: {}", _part1ans, _part2ans);
}
