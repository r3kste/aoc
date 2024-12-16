fn summation(n: usize) -> usize {
    return (n * (n + 1)) / 2;
}

fn part1(input_file: &String) -> usize {
    let mut result = 0;
    let mut stuff: Vec<(i32, i32)> = Vec::new();
    let mut prefix_sum: Vec<i32> = Vec::new();
    prefix_sum.push(0);

    let line = input_file.lines().next().unwrap();
    for (i, c) in line.chars().enumerate() {
        prefix_sum.push(prefix_sum[i] + c as i32 - ('0' as i32));
    }

    let mut file_id = 0;
    for i in (0..(line.len())).step_by(2) {
        stuff.push((file_id, line.chars().nth(i).unwrap() as i32 - ('0' as i32)));
        if i + 1 == line.len() {
            break;
        }
        stuff.push((-1, line.chars().nth(i + 1).unwrap() as i32 - ('0' as i32)));
        file_id += 1;
    }

    let mut space_idx = 1;
    let mut file_idx = stuff.len() - 1;

    while space_idx < file_idx {
        let (_, space_size) = stuff[space_idx];
        let (file_id, file_size) = stuff[file_idx];

        if space_size >= file_size {
            stuff[file_idx] = (-1, file_size);
            stuff.insert(space_idx, (file_id, file_size));
            space_idx += 1;
            file_idx -= 1;
            stuff[space_idx].1 -= file_size;
            if stuff[space_idx].1 == 0 {
                stuff.remove(space_idx);
                space_idx += 1;
                file_idx -= 1;
            }
        } else {
            stuff[space_idx] = (file_id, space_size);
            stuff[file_idx].1 -= space_size;
            space_idx += 2;
        }
    }

    let mut done = 0;
    for (file_id, file_size) in stuff.iter() {
        let file_id = *file_id;
        let file_size = *file_size as usize;

        if file_id == -1 {
            break;
        }

        if done != 0 {
            result += (file_id as usize) * (summation(done + file_size - 1) - summation(done - 1));
        }
        done += file_size;
    }

    result
}

fn part2(input_file: &String) -> usize {
    let mut result = 0;
    let mut stuff: Vec<(i32, i32)> = Vec::new();
    let mut prefix_sum: Vec<i32> = Vec::new();
    prefix_sum.push(0);

    let line = input_file.lines().next().unwrap();
    for (i, c) in line.chars().enumerate() {
        prefix_sum.push(prefix_sum[i] + c as i32 - ('0' as i32));
    }

    let mut file_id = 0;
    for i in (0..(line.len())).step_by(2) {
        stuff.push((file_id, line.chars().nth(i).unwrap() as i32 - ('0' as i32)));
        if i + 1 == line.len() {
            break;
        }
        stuff.push((-1, line.chars().nth(i + 1).unwrap() as i32 - ('0' as i32)));
        file_id += 1;
    }

    let mut file_idx = stuff.len() - 1;
    let mut done_id = i32::MAX;

    while file_idx > 0 {
        let (file_id, file_size) = stuff[file_idx];
        if file_id >= done_id {
            file_idx -= 1;
            while file_idx > 0 && stuff[file_idx].0 == -1 {
                file_idx -= 1;
            }
            continue;
        }

        let mut space_idx = 0;
        for (idx, (id, size)) in stuff.iter().enumerate() {
            if idx > file_idx {
                break;
            }

            let id = *id;
            let size = *size;

            if id != -1 {
                continue;
            }
            if size >= file_size {
                space_idx = idx;
                break;
            }
        }

        done_id = file_id;

        if space_idx == 0 {
            if file_idx >= 2 {
                file_idx -= 2;
            }
            continue;
        }

        stuff[file_idx] = (-1, file_size);
        stuff.insert(space_idx, (file_id, file_size));
        space_idx += 1;
        file_idx -= 1;
        stuff[space_idx].1 -= file_size;
        if stuff[space_idx].1 == 0 {
            stuff.remove(space_idx);
            file_idx -= 1;
        }
    }

    let mut done = 0;
    for (file_id, file_size) in stuff.iter() {
        let file_id = *file_id;
        let file_size = *file_size as usize;

        if file_id <= 0 {
            done += file_size;
            continue;
        }

        if done != 0 {
            result += (file_id as usize) * (summation(done + file_size - 1) - summation(done - 1));
        }
        done += file_size;
    }

    result
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join(".inputs")
            .join("day09.txt"),
    )
    .expect("File Not Found!");

    let _part1ans: usize = part1(&input_file);
    println!("Day 9\nPart 1: {}", _part1ans);

    let _part2ans: usize = part2(&input_file);
    println!("Part 2: {}", _part2ans);
}
