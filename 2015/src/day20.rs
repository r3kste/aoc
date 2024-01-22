use itertools::enumerate;

fn part1(input_file: &String) -> i32 {
    const SIZE: usize = 1000000;
    let mut houses: Vec<usize> = vec![0; SIZE];
    let n = input_file.parse::<usize>().unwrap() / 10;
    for elf in 1..SIZE {
        let presents = elf * 10;
        for house in (elf..SIZE).step_by(elf) {
            houses[house] += presents;
        }
    }
    for (houseno, house) in enumerate(houses) {
        if house >= n * 10 {
            return houseno.try_into().unwrap();
        }
    }
    return 0;
}

fn part2(input_file: &String) -> i32 {
    const SIZE: usize = 1000000;
    let mut houses: Vec<usize> = vec![0; SIZE + 1];
    let n = input_file.parse::<usize>().unwrap();
    for elf in 1..SIZE {
        let presents = elf * 11;
        for house in (elf..=50 * elf).step_by(elf) {
            if house >= SIZE {
                break;
            }
            houses[house] += presents;
        }
    }
    for (houseno, house) in enumerate(houses) {
        if house >= n {
            return houseno.try_into().unwrap();
        }
    }
    return 0;
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day20.txt"),
    )
    .expect("File Not Found!");

    let _part1ans: i32 = part1(&input_file);
    println!("Day 20\nPart 1: {}", _part1ans);

    let _part2ans: i32 = part2(&input_file);
    println!("Part 2: {}", _part2ans);
}
