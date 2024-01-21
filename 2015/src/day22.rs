use std::{
    cmp::{max, min},
    collections::HashMap,
};

const MAX: i32 = 1147483647;

fn f(
    spells: &HashMap<&str, HashMap<&str, i32>>,
    mut boss_stats: (i32, i32),
    mut player_stats: (i32, i32),
    shield_tick: i32,
    poison_tick: i32,
    recharge_tick: i32,
    turn: i32,
    remaining_moves: i32,
    life_drain: i32,
) -> i32 {
    boss_stats.0 -= if poison_tick > 0 {
        spells["Poison"]["Damage"]
    } else {
        0
    };
    let armor = if shield_tick > 0 {
        spells["Shield"]["Armor"]
    } else {
        0
    };
    player_stats.1 += if recharge_tick > 0 {
        spells["Recharge"]["Recharge"]
    } else {
        0
    };
    if (player_stats.0 <= 0) || (player_stats.1 <= 0 || remaining_moves <= 0) {
        return MAX;
    }
    if boss_stats.0 <= 0 {
        return 0;
    }
    let poison_tick = poison_tick - if poison_tick > 0 { 1 } else { 0 };
    let shield_tick = shield_tick - if shield_tick > 0 { 1 } else { 0 };
    let recharge_tick = recharge_tick - if recharge_tick > 0 { 1 } else { 0 };
    return if turn == 1 {
        let mut new_player_stats = player_stats;
        new_player_stats.0 -= max(1, boss_stats.1 - armor);
        f(
            spells,
            boss_stats,
            new_player_stats,
            shield_tick,
            poison_tick,
            recharge_tick,
            turn ^ 1,
            remaining_moves - 1,
            life_drain,
        )
    } else {
        player_stats.0 -= life_drain;
        if player_stats.0 <= 0 {
            return MAX;
        }
        let mut min_cost = MAX;
        let mana = player_stats.1;
        if mana < 53 {
            return MAX;
        }
        if mana >= 53 {
            let mut new_boss_stats = boss_stats;
            new_boss_stats.0 -= spells["Magic Missile"]["Damage"];
            let mut new_player_stats = player_stats;
            new_player_stats.1 -= spells["Magic Missile"]["Cost"];
            min_cost = min(
                min_cost,
                53 + f(
                    spells,
                    new_boss_stats,
                    new_player_stats,
                    shield_tick,
                    poison_tick,
                    recharge_tick,
                    turn ^ 1,
                    remaining_moves - 1,
                    life_drain,
                ),
            );
        }
        if mana >= 73 {
            let mut new_boss_stats = boss_stats;
            new_boss_stats.0 -= spells["Drain"]["Damage"];
            let mut new_player_stats = player_stats;
            new_player_stats.0 += spells["Drain"]["Health"];
            new_player_stats.1 -= spells["Drain"]["Cost"];
            min_cost = min(
                min_cost,
                73 + f(
                    spells,
                    new_boss_stats,
                    new_player_stats,
                    shield_tick,
                    poison_tick,
                    recharge_tick,
                    turn ^ 1,
                    remaining_moves - 1,
                    life_drain,
                ),
            );
        }
        if mana >= 113 && shield_tick == 0 {
            let mut new_player_stats = player_stats;
            new_player_stats.1 -= spells["Shield"]["Cost"];
            min_cost = min(
                min_cost,
                113 + f(
                    spells,
                    boss_stats,
                    new_player_stats,
                    spells["Shield"]["Ticks"],
                    poison_tick,
                    recharge_tick,
                    turn ^ 1,
                    remaining_moves - 1,
                    life_drain,
                ),
            );
        }
        if mana >= 173 && poison_tick == 0 {
            let mut new_player_stats = player_stats;
            new_player_stats.1 -= spells["Poison"]["Cost"];
            min_cost = min(
                min_cost,
                173 + f(
                    spells,
                    boss_stats,
                    new_player_stats,
                    shield_tick,
                    spells["Poison"]["Ticks"],
                    recharge_tick,
                    turn ^ 1,
                    remaining_moves - 1,
                    life_drain,
                ),
            );
        }
        if mana >= 229 && recharge_tick == 0 {
            let mut new_player_stats = player_stats;
            new_player_stats.1 -= spells["Recharge"]["Cost"];
            min_cost = min(
                min_cost,
                229 + f(
                    spells,
                    boss_stats,
                    new_player_stats,
                    shield_tick,
                    poison_tick,
                    spells["Recharge"]["Ticks"],
                    turn ^ 1,
                    remaining_moves - 1,
                    life_drain,
                ),
            );
        }
        min_cost
    }
}

fn part1n2(input_file: &String) -> (i32, i32) {
    let mut spells: HashMap<&str, HashMap<&str, i32>> = HashMap::new();

    spells.insert("Magic Missile", HashMap::new());
    let temp = spells.get_mut("Magic Missile").unwrap();
    temp.insert("Cost", 53);
    temp.insert("Damage", 4);

    spells.insert("Drain", HashMap::new());
    let temp = spells.get_mut("Drain").unwrap();
    temp.insert("Cost", 73);
    temp.insert("Damage", 2);
    temp.insert("Health", 2);

    spells.insert("Shield", HashMap::new());
    let temp = spells.get_mut("Shield").unwrap();
    temp.insert("Cost", 113);
    temp.insert("Armor", 7);
    temp.insert("Ticks", 6);

    spells.insert("Poison", HashMap::new());
    let temp = spells.get_mut("Poison").unwrap();
    temp.insert("Cost", 173);
    temp.insert("Damage", 3);
    temp.insert("Ticks", 6);

    spells.insert("Recharge", HashMap::new());
    let temp = spells.get_mut("Recharge").unwrap();
    temp.insert("Cost", 229);
    temp.insert("Recharge", 101);
    temp.insert("Ticks", 5);

    let mut boss_stats: Vec<i32> = Vec::new();
    for stat in input_file.lines() {
        let words: Vec<&str> = stat.split_whitespace().collect();
        boss_stats.push(words.last().unwrap().parse::<i32>().unwrap());
    }

    return (
        f(
            &spells,
            (boss_stats[0], boss_stats[1]),
            (50, 500),
            0,
            0,
            0,
            0,
            20,
            0,
        ),
        f(
            &spells,
            (boss_stats[0], boss_stats[1]),
            (50, 500),
            0,
            0,
            0,
            0,
            20,
            1,
        ),
    );
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day22.txt"),
    )
    .expect("File Not Found!");

    let (_part1ans, _part2ans): (i32, i32) = part1n2(&input_file);
    println!("Day 22\nPart 1: {}\nPart 2: {}", _part1ans, _part2ans);
}
