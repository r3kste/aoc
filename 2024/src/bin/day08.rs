use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    x: usize,
    y: usize,
}

impl Node {
    fn within_bounds(&self, n_rows: usize, n_cols: usize) -> bool {
        self.x < n_rows && self.y < n_cols
    }
}
fn part1(grid: &Vec<Vec<char>>) -> i32 {
    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let mut antinodes: HashSet<Node> = HashSet::new();

    let mut map: HashMap<char, Vec<Node>> = HashMap::new();
    for i in 0..n_rows {
        for j in 0..n_cols {
            let c: char = grid[i][j];
            if c == '.' {
                continue;
            }
            match map.get_mut(&c) {
                Some(nodes) => {
                    nodes.push(Node { x: i, y: j });
                }
                None => {
                    map.insert(c, vec![Node { x: i, y: j }]);
                }
            }
        }
    }

    for (_, nodes) in map.iter() {
        let n = nodes.len();
        for node1_idx in 0..n {
            let node1 = nodes[node1_idx];
            for node2_idx in (node1_idx + 1)..n {
                let node2 = nodes[node2_idx];
                let d12x = node2.x as i32 - node1.x as i32;
                let d12y = node2.y as i32 - node1.y as i32;

                let antinode12 = Node {
                    x: (node1.x as i32 - d12x) as usize,
                    y: (node1.y as i32 - d12y) as usize,
                };
                let antinode21 = Node {
                    x: (node2.x as i32 + d12x) as usize,
                    y: (node2.y as i32 + d12y) as usize,
                };

                if antinode12.within_bounds(n_rows, n_cols) {
                    antinodes.insert(antinode12);
                }
                if antinode21.within_bounds(n_rows, n_cols) {
                    antinodes.insert(antinode21);
                }
            }
        }
    }

    return antinodes.len() as i32;
}

fn part2(grid: &Vec<Vec<char>>) -> i32 {
    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let mut antinodes: HashSet<Node> = HashSet::new();

    let mut map: HashMap<char, Vec<Node>> = HashMap::new();
    for i in 0..n_rows {
        for j in 0..n_cols {
            let c: char = grid[i][j];
            if c == '.' {
                continue;
            }
            match map.get_mut(&c) {
                Some(nodes) => {
                    nodes.push(Node { x: i, y: j });
                }
                None => {
                    map.insert(c, vec![Node { x: i, y: j }]);
                }
            }
        }
    }

    for (_, nodes) in map.iter() {
        let n = nodes.len();
        for node1_idx in 0..n {
            let node1 = nodes[node1_idx];
            for node2_idx in (node1_idx + 1)..n {
                let node2 = nodes[node2_idx];
                let d12x = node2.x as i32 - node1.x as i32;
                let d12y = node2.y as i32 - node1.y as i32;

                let mut antinode12 = Node {
                    x: node1.x,
                    y: node1.y,
                };

                loop {
                    antinodes.insert(antinode12);
                    let (new_x, new_y) = (antinode12.x as i32 - d12x, antinode12.y as i32 - d12y);
                    if new_x < 0 || new_y < 0 || new_x >= n_rows as i32 || new_y >= n_cols as i32 {
                        break;
                    }

                    antinode12 = Node {
                        x: new_x as usize,
                        y: new_y as usize,
                    };
                }

                let mut antinode21 = Node {
                    x: node2.x,
                    y: node2.y,
                };

                loop {
                    antinodes.insert(antinode21);
                    let (new_x, new_y) = (antinode21.x as i32 + d12x, antinode21.y as i32 + d12y);
                    if new_x < 0 || new_y < 0 || new_x >= n_rows as i32 || new_y >= n_cols as i32 {
                        break;
                    }

                    antinode21 = Node {
                        x: new_x as usize,
                        y: new_y as usize,
                    };
                }
            }
        }
    }

    return antinodes.len() as i32;
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join(".inputs")
            .join("day08.txt"),
    )
    .expect("File Not Found!");

    let grid: Vec<Vec<char>> = input_file
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start = std::time::Instant::now();
    let _part1ans = part1(&grid);
    let duration = start.elapsed();
    println!("Day 8\nPart 1: {}\t(took {:?})", _part1ans, duration);

    let start = std::time::Instant::now();
    let _part2ans = part2(&grid);
    let duration = start.elapsed();
    println!("Part 2: {}\t(took {:?})", _part2ans, duration);
}
