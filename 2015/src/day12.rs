use serde_json::Value;

fn sum(data: &Value, ignore: bool) -> i32 {
    match data {
        Value::Number(num) => num.as_i64().unwrap() as i32,
        Value::Array(arr) => arr.iter().map(|value| sum(value, ignore)).sum(),
        Value::Object(obj) => {
            if ignore && obj.values().any(|value| value == "red") {
                0
            } else {
                obj.values().map(|value| sum(value, ignore)).sum()
            }
        }
        _ => 0,
    }
}

fn part1(input_file: &Value) -> i32 {
    return sum(&input_file, false);
}

fn part2(input_file: &Value) -> i32 {
    return sum(&input_file, true);
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day12.txt"),
    )
    .expect("File Not Found!");
    let data: Value = serde_json::from_str(&input_file).unwrap();
    let _part1ans: i32 = part1(&data);
    println!("Day 12\nPart 1: {}", _part1ans);

    let _part2ans: i32 = part2(&data);
    println!("Part 2: {}", _part2ans);
}
