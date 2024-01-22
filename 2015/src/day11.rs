fn valid(password: &Vec<char>) -> bool {
    let mut consecutive = 0;
    let mut increasing = false;
    for (i, char) in password.iter().enumerate() {
        let char = char.to_owned();
        if char == 'i' || char == 'o' || char == 'l' {
            return false;
        }
        if i > 1
            && (char as u8) == (password[i - 1] as u8 + 1)
            && (password[i - 1] as u8) == (password[i - 2] as u8 + 1)
        {
            increasing = true;
        }
    }
    let mut i = 0;
    while i < password.len() {
        let char = password[i].to_owned();
        if i > 0 && char == password[i - 1] {
            consecutive += 1;
            i += 1;
        }
        i += 1;
    }
    return consecutive == 2 && increasing;
}

fn next_password(password: &mut Vec<char>, index: usize) -> Vec<char> {
    return if password[index] != 'z' {
        password[index] = (password[index] as u8 + 1) as char;
        if password[index] == 'i' || password[index] == 'o' || password[index] == 'l' {
            password[index] = (password[index] as u8 + 1) as char;
        }
        password.to_vec()
    } else {
        password[index] = 'a';
        next_password(password, index - 1)
    };
}

fn part1n2(input_file: &String) -> (String, String) {
    let mut _part1ans = String::new();
    let mut _part2ans = String::new();
    let mut input_file: Vec<char> = input_file.chars().collect();
    let n = input_file.len();
    loop {
        let new_password = next_password(&mut input_file, n - 1);
        if valid(&new_password) {
            _part1ans = new_password.iter().collect();
            input_file = new_password;
            loop {
                let new_password = next_password(&mut input_file, n - 1);
                if valid(&new_password) {
                    _part2ans = new_password.iter().collect();
                    return (_part1ans, _part2ans);
                }
            }
        }
    }
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day11.txt"),
    )
    .expect("File Not Found!");

    let (_part1ans, _part2ans) = part1n2(&input_file);
    println!("Day 11\nPart 1: {}\nPart 2: {}", _part1ans, _part2ans);
}
