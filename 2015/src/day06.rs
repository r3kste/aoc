fn part1(input_file: &String) -> i32 {
    const N: usize = 1000;
    let mut grid: Vec<Vec<i32>> = vec![vec![0; N]; N];
    for instruction in input_file.lines() {
        let mut words = instruction.split_whitespace();
        let mut typ = words.next().unwrap();
        if typ.eq("turn") {
            typ = words.next().unwrap();
        }
        let start = words.next().unwrap();
        words.next();
        let end = words.next().unwrap();

        let mut start = start.split(',');
        let start_coords: (i32, i32) = (
            start.next().unwrap().parse().unwrap(),
            start.next().unwrap().parse().unwrap(),
        );

        let mut end = end.split(',');
        let end_coords: (i32, i32) = (
            end.next().unwrap().parse().unwrap(),
            end.next().unwrap().parse().unwrap(),
        );

        for i in start_coords.1..=end_coords.1 {
            for j in start_coords.0..=end_coords.0 {
                let i: usize = i.try_into().unwrap();
                let j: usize = j.try_into().unwrap();
                let current = grid[i][j];
                if typ.eq("toggle") {
                    if current == 1 {
                        grid[i][j] = 0;
                    } else {
                        grid[i][j] = 1;
                    }
                } else if typ.eq("off") {
                    grid[i][j] = 0;
                } else {
                    grid[i][j] = 1;
                }
            }
        }
    }
    let mut count = 0;
    for i in 0..N {
        for j in 0..N {
            if grid[i][j] == 1 {
                count += 1;
            }
        }
    }
    return count;
}

fn part2(input_file: &String) -> i32 {
    const N: usize = 1000;
    let mut grid: Vec<Vec<i32>> = vec![vec![0; N]; N];
    for instruction in input_file.lines() {
        let mut words = instruction.split_whitespace();
        let mut typ = words.next().unwrap();
        if typ.eq("turn") {
            typ = words.next().unwrap();
        }
        let start = words.next().unwrap();
        words.next();
        let end = words.next().unwrap();

        let mut start = start.split(',');
        let start_coords: (i32, i32) = (
            start.next().unwrap().parse().unwrap(),
            start.next().unwrap().parse().unwrap(),
        );

        let mut end = end.split(',');
        let end_coords: (i32, i32) = (
            end.next().unwrap().parse().unwrap(),
            end.next().unwrap().parse().unwrap(),
        );

        for i in start_coords.1..=end_coords.1 {
            for j in start_coords.0..=end_coords.0 {
                let i: usize = i.try_into().unwrap();
                let j: usize = j.try_into().unwrap();
                if typ.eq("toggle") {
                    grid[i][j] += 2;
                } else if typ.eq("off") {
                    grid[i][j] = std::cmp::max(grid[i][j] - 1, 0);
                } else {
                    grid[i][j] += 1;
                }
            }
        }
    }
    let mut count = 0;
    for i in 0..N {
        for j in 0..N {
            count += grid[i][j];
        }
    }
    return count;
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day06.txt"),
    )
    .expect("File Not Found!");

    let _part1ans: i32 = part1(&input_file);
    let _part2ans: i32 = part2(&input_file);

    println!("Day 06\nPart 1: {}\nPart 2: {}", _part1ans, _part2ans);
}
