fn update(cur_grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = cur_grid.len();
    let mut new_grid = Vec::clone(cur_grid);

    for i in 0..n {
        for j in 0..n {
            let top_left_i = if i == 0 { 0 } else { i - 1 };
            let top_left_j = if j == 0 { 0 } else { j - 1 };
            let bot_right_i = if i == n - 1 { n - 1 } else { i + 1 };
            let bot_right_j = if j == n - 1 { n - 1 } else { j + 1 };

            let mut count = 0;
            let cur_state = cur_grid[i][j];

            for surr_i in top_left_i..=bot_right_i {
                for surr_j in top_left_j..=bot_right_j {
                    if surr_i == i && surr_j == j {
                        continue;
                    }
                    if cur_grid[surr_i][surr_j] == '#' {
                        count += 1;
                    }
                }
            }

            if cur_state == '#' {
                if count != 2 && count != 3 {
                    new_grid[i][j] = '.';
                }
            } else {
                if count == 3 {
                    new_grid[i][j] = '#';
                }
            }
        }
    }
    return new_grid;
}

fn part1(input_file: &Vec<Vec<char>>) -> i32 {
    const STEPS: i32 = 100;
    let mut grid = input_file.clone().to_owned();

    for _ in 0..STEPS {
        grid = update(&grid);
    }
    let count = grid.iter().flatten().filter(|&&x| x == '#').count();
    return count.try_into().unwrap();
}

fn part2(input_file: &Vec<Vec<char>>) -> i32 {
    const STEPS: i32 = 100;
    let mut grid = input_file.clone().to_owned();
    let n = grid.len();

    for _ in 0..STEPS {
        grid[0][0] = '#';
        grid[0][n - 1] = '#';
        grid[n - 1][0] = '#';
        grid[n - 1][n - 1] = '#';
        grid = update(&grid);
    }
    grid[0][0] = '#';
    grid[0][n - 1] = '#';
    grid[n - 1][0] = '#';
    grid[n - 1][n - 1] = '#';

    let count = grid.iter().flatten().filter(|&&x| x == '#').count();
    return count.try_into().unwrap();
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day18.txt"),
    )
    .expect("File Not Found!");

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input_file.lines() {
        grid.push(line.chars().collect());
    }

    let _part1ans: i32 = part1(&grid);
    println!("Day 18\nPart 1: {}", _part1ans);

    let _part2ans: i32 = part2(&grid);
    println!("Part 2: {}", _part2ans);
}
