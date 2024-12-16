use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self, turn: Direction) -> Direction {
        match (self, turn) {
            (Direction::Up, Direction::Left) => Direction::Left,
            (Direction::Up, Direction::Right) => Direction::Right,
            (Direction::Down, Direction::Left) => Direction::Right,
            (Direction::Down, Direction::Right) => Direction::Left,
            (Direction::Left, Direction::Left) => Direction::Down,
            (Direction::Left, Direction::Right) => Direction::Up,
            (Direction::Right, Direction::Left) => Direction::Up,
            (Direction::Right, Direction::Right) => Direction::Down,
            _ => panic!("Invalid Turn"),
        }
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Direction::Up, Direction::Up) => true,
            (Direction::Down, Direction::Down) => true,
            (Direction::Left, Direction::Left) => true,
            (Direction::Right, Direction::Right) => true,
            _ => false,
        }
    }
}

impl Eq for Direction {}

impl std::hash::Hash for Direction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Direction::Up => 0.hash(state),
            Direction::Down => 1.hash(state),
            Direction::Left => 2.hash(state),
            Direction::Right => 3.hash(state),
        }
    }
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

fn step(
    pos: (usize, usize, Direction),
    grid: &Vec<Vec<char>>,
) -> Option<(usize, usize, Direction)> {
    let (x, y, dir) = pos;
    match dir {
        Direction::Up => {
            if x == 0 {
                return None;
            }
            Some((x - 1, y, dir))
        }
        Direction::Down => {
            if x == grid.len() - 1 {
                return None;
            }
            Some((x + 1, y, dir))
        }
        Direction::Left => {
            if y == 0 {
                return None;
            }
            Some((x, y - 1, dir))
        }
        Direction::Right => {
            if y == grid[x].len() - 1 {
                return None;
            }
            Some((x, y + 1, dir))
        }
    }
}

fn simulate(grid: &mut Vec<Vec<char>>) -> bool {
    // Returns true if guard has left the grid
    // Returns false if guard is stuck in a loop
    let mut guard_pos = (0, 0, Direction::Up);

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let c = grid[i][j];
            match c {
                '^' => guard_pos = (i, j, Direction::Up),
                'v' => guard_pos = (i, j, Direction::Down),
                '<' => guard_pos = (i, j, Direction::Left),
                '>' => guard_pos = (i, j, Direction::Right),
                _ => {}
            }
            if guard_pos != (0, 0, Direction::Up) {
                break;
            }
        }
        if guard_pos != (0, 0, Direction::Up) {
            break;
        }
    }

    let mut step_tracker: HashSet<(usize, usize, Direction)> = HashSet::new();

    grid[guard_pos.0][guard_pos.1] = 'X';
    step_tracker.insert(guard_pos);

    loop {
        let (mut new_x, mut new_y, mut new_dir);

        match step(guard_pos, &grid) {
            Some((x, y, dir)) => {
                (new_x, new_y, new_dir) = (x, y, dir);
            }
            None => return true,
        }

        if grid[new_x][new_y] == '#' {
            (new_x, new_y) = (guard_pos.0, guard_pos.1);
            new_dir = new_dir.turn(Direction::Right);
        }
        grid[new_x][new_y] = 'X';
        guard_pos = (new_x, new_y, new_dir);

        if step_tracker.contains(&guard_pos) {
            return false;
        }
        step_tracker.insert(guard_pos);
    }
}

fn part1(grid: &Vec<Vec<char>>) -> i32 {
    let mut result = 0;
    let mut grid = grid.clone();

    simulate(&mut grid);

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'X' {
                result += 1;
            }
        }
    }

    return result;
}

fn part2(grid: &Vec<Vec<char>>) -> i32 {
    let mut result = 0;
    let mut grid_copy = grid.clone();

    simulate(&mut grid_copy);

    let mut obstacle_positions: Vec<(usize, usize)> = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid_copy[i][j] == 'X' {
                obstacle_positions.push((i, j));
            }
        }
    }

    for (obstacle_x, obstacle_y) in obstacle_positions {
        let mut grid_copy = grid.clone();
        grid_copy[obstacle_x][obstacle_y] = '#';

        if !simulate(&mut grid_copy) {
            result += 1;
        }
    }

    return result;
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join(".inputs")
            .join("day06.txt"),
    )
    .expect("File Not Found!");

    let grid: Vec<Vec<char>> = input_file
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let _part1ans: i32 = part1(&grid);
    println!("Day 6\nPart 1: {}", _part1ans);

    let _part2ans: i32 = part2(&grid);
    println!("Part 2: {}", _part2ans);
}
