fn horizontal_fsearch(grid: &Vec<Vec<char>>, word: &str) -> i32 {
    let mut result = 0;
    let n = grid.len();
    let m = grid[0].len();
    for i in 0..n {
        for j in 0..m {
            if j + word.len() <= m {
                let mut found = true;
                for k in 0..word.len() {
                    if grid[i][j + k] != word.chars().nth(k).unwrap() {
                        found = false;
                        break;
                    }
                }
                if found {
                    result += 1;
                }
            }
        }
    }
    return result;
}

fn vertical_fsearch(grid: &Vec<Vec<char>>, word: &str) -> i32 {
    let mut result = 0;
    let n = grid.len();
    let m = grid[0].len();
    for i in 0..n {
        for j in 0..m {
            if i + word.len() <= n {
                let mut found = true;
                for k in 0..word.len() {
                    if grid[i + k][j] != word.chars().nth(k).unwrap() {
                        found = false;
                        break;
                    }
                }
                if found {
                    result += 1;
                }
            }
        }
    }
    return result;
}

fn diagonal_fsearch(grid: &Vec<Vec<char>>, word: &str) -> i32 {
    let mut result = 0;
    let n = grid.len();
    let m = grid[0].len();
    for i in 0..n {
        for j in 0..m {
            if i + word.len() <= n && j + word.len() <= m {
                let mut found = true;
                for k in 0..word.len() {
                    if grid[i + k][j + k] != word.chars().nth(k).unwrap() {
                        found = false;
                        break;
                    }
                }
                if found {
                    result += 1;
                }
            }
        }
    }
    return result;
}

fn diagonal_rsearch(grid: &Vec<Vec<char>>, word: &str) -> i32 {
    let mut result = 0;
    let n = grid.len();
    let m = grid[0].len();
    for i in 0..n {
        for j in (word.len() - 1)..m {
            if i + word.len() <= n {
                let mut found = true;
                for k in 0..word.len() {
                    if grid[i + k][j - k] != word.chars().nth(k).unwrap() {
                        found = false;
                        break;
                    }
                }
                if found {
                    result += 1;
                }
            }
        }
    }
    return result;
}

fn diagonal_xsearch(grid: &Vec<Vec<char>>, word: &str) -> i32 {
    let word_rev: &str = &word.chars().rev().collect::<String>();
    let l = (word.len() - 1) / 2;

    let mut result = 0;
    let n = grid.len();
    let m = grid[0].len();
    for i in l..n - l {
        for j in l..m - l {
            let mut found_diag1 = false;
            let mut found_diag2 = false;

            for k in 0..word.len() {
                if grid[i - l + k][j - l + k] == word.chars().nth(k).unwrap() {
                    found_diag1 = true;
                } else {
                    found_diag1 = false;
                    break;
                }
            }
            if !found_diag1 {
                for k in 0..word.len() {
                    if grid[i - l + k][j - l + k] == word_rev.chars().nth(k).unwrap() {
                        found_diag1 = true;
                    } else {
                        found_diag1 = false;
                        break;
                    }
                }
            }

            for k in 0..word.len() {
                if grid[i - l + k][j + l - k] == word.chars().nth(k).unwrap() {
                    found_diag2 = true;
                } else {
                    found_diag2 = false;
                    break;
                }
            }
            if !found_diag2 {
                for k in 0..word.len() {
                    if grid[i - l + k][j + l - k] == word_rev.chars().nth(k).unwrap() {
                        found_diag2 = true;
                    } else {
                        found_diag2 = false;
                        break;
                    }
                }
            }

            if found_diag1 && found_diag2 {
                result += 1;
            }
        }
    }
    return result;
}

fn part1(grid: &Vec<Vec<char>>) -> i32 {
    let word = "XMAS";
    let word_rev = "SAMX";
    let mut result = 0;

    result += horizontal_fsearch(&grid, word);
    result += horizontal_fsearch(&grid, word_rev);
    result += vertical_fsearch(&grid, word);
    result += vertical_fsearch(&grid, word_rev);

    result += diagonal_fsearch(&grid, word);
    result += diagonal_fsearch(&grid, word_rev);
    result += diagonal_rsearch(&grid, word);
    result += diagonal_rsearch(&grid, word_rev);

    return result;
}

fn part2(grid: &Vec<Vec<char>>) -> i32 {
    let word = "MAS";
    let mut result = 0;

    result += diagonal_xsearch(&grid, word);
    return result;
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join(".inputs")
            .join("day04.txt"),
    )
    .expect("File Not Found!");

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input_file.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    let _part1ans: i32 = part1(&grid);
    println!("Day 4\nPart 1: {}", _part1ans);

    let _part2ans: i32 = part2(&grid);
    println!("Part 2: {}", _part2ans);
}
