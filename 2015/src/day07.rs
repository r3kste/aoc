use std::collections::{HashMap, HashSet};

fn _graphviz(input_file: &String) {
    for line in input_file.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() == 3 {
            let node = words.get(0).unwrap();
            let target = words.get(2).unwrap();
            println!("{} -> {}", node, target);
        } else if words.len() == 4 {
            // NOT node -> target
            let node = words.get(1).unwrap();
            let target = words.get(3).unwrap();
            println!("{} -> {} [color = \"red\"]", node, target);
        } else if words.len() == 5 {
            let node1 = words.get(0).unwrap();
            let typ = words.get(1).unwrap();
            let node2 = words.get(2).unwrap();
            let target = words.get(4).unwrap();
            let mut _color = "";
            if typ == &("LSHIFT") {
                _color = "green";
            } else if typ.eq(&"RSHIFT") {
                _color = "blue";
            } else if typ.eq(&"AND") {
                _color = "orange";
            } else {
                _color = "yellow";
            }
            if node1.parse::<i32>().is_ok() {
                println!("{} -> {} [color = \"{}\"]", node2, target, _color);
            } else if node2.parse::<i32>().is_ok() {
                println!("{} -> {} [color = \"{}\"]", node1, target, _color);
            } else {
                println!("{} -> {} [color = \"{}\"]", node1, target, _color);
                println!("{} -> {} [color = \"{}\"]", node2, target, _color);
            }
        }
    }
}

fn sort_file(input_file: &String) -> Vec<Vec<&str>> {
    let mut que: Vec<Vec<&str>> = Vec::new();
    for line in input_file.lines() {
        let words = line.split_whitespace().collect();
        que.push(words);
    }
    let mut sorted_file: Vec<Vec<&str>> = Vec::new();
    let mut seen: HashSet<&str> = HashSet::new();

    while !seen.contains("a") {
        let n: i32 = que[0].len().try_into().unwrap();
        let target = que[0][que[0].len() - 1];
        if n == 3 && que[0][0].parse::<i32>().is_ok() {
            sorted_file.push(que[0].to_owned());
            seen.insert(target);
            que.remove(0);
        } else {
            let mut nodes: HashSet<&str> = HashSet::new();

            for i in 0..(n - 2) {
                let i: usize = i.try_into().unwrap();
                let parts = que[0][i];
                if parts.chars().all(|f| f.is_lowercase()) {
                    nodes.insert(parts);
                }
            }

            if nodes.is_subset(&seen) {
                sorted_file.push(que[0].to_owned());
                seen.insert(target);
                que.remove(0);
            } else {
                que.push(que[0].to_owned());
                que.remove(0);
            }
        }
    }
    return sorted_file;
}

fn part1(input_file: &String) -> i32 {
    let mut circuit: HashMap<&str, u16> = HashMap::new();
    for operation in input_file.lines() {
        let operation: Vec<&str> = operation.split(" ").collect();
        let n = operation.len() - 1;
        let target = operation[n - 1];

        let mut nodes: Vec<&str> = Vec::new();
        let mut typ = "";
        let mut num: u16 = 0;

        for i in 0..(n - 2) {
            let parts = operation[i];
            if parts.eq("->") {
                continue;
            } else if parts.chars().all(|f| f.is_lowercase()) {
                nodes.push(parts);
            } else if parts.chars().all(|f| f.is_uppercase()) {
                typ = parts;
            } else {
                num = parts.parse::<u16>().unwrap();
            }
        }
        if typ == "" {
            if nodes.len() > 0 {
                circuit.insert(target, circuit[nodes[0]]);
            } else {
                circuit.insert(target, num);
            }
        } else if typ == "NOT" {
            circuit.insert(target, !(circuit[nodes[0]]));
        } else if typ == "AND" {
            let mut a = 0;
            let mut b = 0;
            if nodes.len() == 2 {
                a = circuit[nodes[0]];
                b = circuit[nodes[1]];
            } else if nodes.len() == 1 {
                a = circuit[nodes[0]];
                b = num;
            }
            circuit.insert(target, a & b);
        } else if typ == "OR" {
            let mut a = 0;
            let mut b = 0;
            if nodes.len() == 2 {
                a = circuit[nodes[0]];
                b = circuit[nodes[1]];
            } else if nodes.len() == 1 {
                a = circuit[nodes[0]];
                b = num;
            }
            circuit.insert(target, a | b);
        } else if typ == "RSHIFT" {
            circuit.insert(target, circuit[nodes[0]] >> num);
        } else if typ == "LSHIFT" {
            circuit.insert(target, circuit[nodes[0]] << num);
        }
    }
    return circuit["a"].into();
}

fn part2(input_file: &String) -> i32 {
    let mut circuit: HashMap<&str, u16> = HashMap::new();
    for operation in input_file.lines() {
        let operation: Vec<&str> = operation.split(" ").collect();
        let n = operation.len() - 1;
        let target = operation[n - 1];

        if target == "b" {
            circuit.insert("b", part1(input_file).try_into().unwrap());
            continue;
        }
        let mut nodes: Vec<&str> = Vec::new();
        let mut typ = "";
        let mut num: u16 = 0;

        for i in 0..(n - 2) {
            let parts = operation[i];
            if parts.eq("->") {
                continue;
            } else if parts.chars().all(|f| f.is_lowercase()) {
                nodes.push(parts);
            } else if parts.chars().all(|f| f.is_uppercase()) {
                typ = parts;
            } else {
                num = parts.parse::<u16>().unwrap();
            }
        }
        if typ == "" {
            if nodes.len() > 0 {
                circuit.insert(target, circuit[nodes[0]]);
            } else {
                circuit.insert(target, num);
            }
        } else if typ == "NOT" {
            circuit.insert(target, !(circuit[nodes[0]]));
        } else if typ == "AND" {
            let mut a = 0;
            let mut b = 0;
            if nodes.len() == 2 {
                a = circuit[nodes[0]];
                b = circuit[nodes[1]];
            } else if nodes.len() == 1 {
                a = circuit[nodes[0]];
                b = num;
            }
            circuit.insert(target, a & b);
        } else if typ == "OR" {
            let mut a = 0;
            let mut b = 0;
            if nodes.len() == 2 {
                a = circuit[nodes[0]];
                b = circuit[nodes[1]];
            } else if nodes.len() == 1 {
                a = circuit[nodes[0]];
                b = num;
            }
            circuit.insert(target, a | b);
        } else if typ == "RSHIFT" {
            circuit.insert(target, circuit[nodes[0]] >> num);
        } else if typ == "LSHIFT" {
            circuit.insert(target, circuit[nodes[0]] << num);
        }
    }
    return circuit["a"].into();
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day07.txt"),
    )
    .expect("File Not Found!");

    // graphviz(&input_file);
    let mut sorted_file = "".to_string();
    for lines in sort_file(&input_file) {
        for words in lines {
            sorted_file += &format!("{} ", words).to_string();
        }
        sorted_file += "\n";
    }

    let _part1ans: i32 = part1(&sorted_file);
    let _part2ans: i32 = part2(&sorted_file);

    println!("Day 07\nPart 1: {}\nPart 2: {}", _part1ans, _part2ans);
}
