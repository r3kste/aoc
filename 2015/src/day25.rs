fn part1(data: &(usize, usize)) -> i32 {
    let row = data.0;
    let col = data.1;
    let lim = row + col;
    let mut codes = vec![vec![0; lim]; lim];
    let (mut i, mut j) = (1, 1);
    codes[1][1] = 20151125;
    let mut prev: u64 = 20151125;
    while codes[row][col] == 0 {
        if i == 1 {
            i = j + 1;
            j = 1;
        } else {
            i -= 1;
            j += 1;
        }
        if codes[i][j] == 0 {
            prev = (prev * 252533) % 33554393;
            codes[i][j] = prev as i32;
        }
    }
    return codes[row][col];
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day25.txt"),
    )
    .expect("File Not Found!");

    let words: Vec<&str> = input_file.split_whitespace().collect();
    let data = (
        words[15]
            .split(',')
            .nth(0)
            .unwrap()
            .parse::<usize>()
            .unwrap(),
        words[17]
            .split('.')
            .nth(0)
            .unwrap()
            .parse::<usize>()
            .unwrap(),
    );
    let _part1ans: i32 = part1(&data);
    println!("Day 25\nPart 1: {}", _part1ans);
}
