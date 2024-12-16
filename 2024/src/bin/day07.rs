fn part1(input_file: &String) -> i128 {
    let mut result = 0;

    for line in input_file.lines() {
        let (res, operands) = line.split_once(": ").unwrap();
        let total_res: i128 = res.parse().unwrap();
        let operands: Vec<i128> = operands.split(" ").map(|x| x.parse().unwrap()).collect();

        let n = operands.len();
        let mut queue: Vec<(usize, i128)> = vec![(n - 2, total_res)];

        while !queue.is_empty() {
            let (last_nan_idx, res) = queue.pop().unwrap();

            if last_nan_idx == 0 {
                if operands[0] + operands[1] == res || operands[0] * operands[1] == res {
                    result += total_res;
                    break;
                }
                continue;
            }

            let num2 = operands[last_nan_idx + 1];
            if num2 > res {
                continue;
            }

            queue.push((last_nan_idx - 1, res - num2));
            if res % num2 == 0 {
                queue.push((last_nan_idx - 1, res / num2));
            }
        }
    }

    return result;
}

fn part2(input_file: &String) -> i128 {
    let mut result = 0;

    for line in input_file.lines() {
        let (res, operands) = line.split_once(": ").unwrap();
        let total_res: i128 = res.parse().unwrap();
        let operands: Vec<i128> = operands.split(" ").map(|x| x.parse().unwrap()).collect();

        let n = operands.len();
        let mut queue: Vec<(usize, i128)> = vec![(n - 2, total_res)];

        while !queue.is_empty() {
            let (last_nan_idx, res) = queue.pop().unwrap();

            if last_nan_idx == 0 {
                if operands[0] + operands[1] == res
                    || operands[0] * operands[1] == res
                    || (format!("{}{}", operands[0], operands[1]) == res.to_string())
                {
                    result += total_res;
                    break;
                }
                continue;
            }

            let num2 = operands[last_nan_idx + 1];
            if num2 > res {
                continue;
            }

            queue.push((last_nan_idx - 1, res - num2));
            if res % num2 == 0 {
                queue.push((last_nan_idx - 1, res / num2));
            }

            let num2_len = num2.to_string().len();
            let res_str = res.to_string();
            let res_len = res_str.len();
            if res_str[res_len - num2_len..] == num2.to_string() {
                let num1 = res_str[..res_len - num2_len].parse().unwrap_or(0);
                queue.push((last_nan_idx - 1, num1));
            }
        }
    }

    return result;
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join(".inputs")
            .join("day07.txt"),
    )
    .expect("File Not Found!");

    let start = std::time::Instant::now();
    let _part1ans: i128 = part1(&input_file);
    let duration = start.elapsed();
    println!("Day 7\nPart 1: {}\t(took {:?})", _part1ans, duration);

    let start = std::time::Instant::now();
    let _part2ans: i128 = part2(&input_file);
    let duration = start.elapsed();
    println!("Part 2: {}\t(took {:?})", _part2ans, duration);
}
