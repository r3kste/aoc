use std::{
    cmp::{max, min},
    collections::HashMap,
    i32::MAX,
};

fn part1n2(input_file: &String) -> (i32, i32) {
    let mut weapons: HashMap<&str, (i32, (i32, i32))> = HashMap::new();
    weapons.insert("Dagger", (8, (4, 0)));
    weapons.insert("Shortsword", (10, (5, 0)));
    weapons.insert("Warhammer", (25, (6, 0)));
    weapons.insert("Longsword", (40, (7, 0)));
    weapons.insert("Greataxe", (74, (8, 0)));

    let mut armors: HashMap<&str, (i32, (i32, i32))> = HashMap::new();
    armors.insert("Nothing", (0, (0, 0)));
    armors.insert("Leather", (13, (0, 1)));
    armors.insert("Chainmail", (31, (0, 2)));
    armors.insert("Splintmail", (53, (0, 3)));
    armors.insert("Bandedmail", (75, (0, 4)));
    armors.insert("Platemail", (102, (0, 5)));

    let mut rings: HashMap<&str, (i32, (i32, i32))> = HashMap::new();
    rings.insert("Nothing", (0, (0, 0)));
    rings.insert("Nothing", (0, (0, 0)));
    rings.insert("Damage +1", (25, (1, 0)));
    rings.insert("Damage +2", (50, (2, 0)));
    rings.insert("Damage +3", (100, (3, 0)));
    rings.insert("Defense +1", (20, (0, 1)));
    rings.insert("Defense +2", (40, (0, 2)));
    rings.insert("Defense +3", (80, (0, 3)));

    let mut boss_stats: Vec<i32> = Vec::new();
    for stat in input_file.lines() {
        let words: Vec<&str> = stat.split_whitespace().collect();
        boss_stats.push(words.last().unwrap().parse::<i32>().unwrap());
    }

    let boss_hp = boss_stats[0];
    let boss_damage = boss_stats[1];
    let boss_armor = boss_stats[2];

    let mut min_cost = MAX;
    let mut max_cost = 0;

    for weapon in weapons {
        for armor in armors.iter() {
            for ring1 in rings.iter() {
                for ring2 in rings.iter() {
                    if ring1.0 == ring2.0 && ring1.0 != &"Nothing" {
                        continue;
                    }
                    let player_hp = 100;
                    let player_damage =
                        weapon.1 .1 .0 + armor.1 .1 .0 + ring1.1 .1 .0 + ring2.1 .1 .0;
                    let player_armor =
                        weapon.1 .1 .1 + armor.1 .1 .1 + ring1.1 .1 .1 + ring2.1 .1 .1;

                    let boss_damage = max(1, boss_damage - player_armor);
                    let player_damage = max(1, player_damage - boss_armor);

                    let req_hits_for_boss = player_hp / boss_damage;
                    let req_hits_for_player = boss_hp / player_damage;

                    if req_hits_for_player <= req_hits_for_boss {
                        min_cost =
                            min(min_cost, weapon.1 .0 + armor.1 .0 + ring1.1 .0 + ring2.1 .0);
                    } else {
                        max_cost = max(max_cost, weapon.1 .0 + armor.1 .0 + ring1.1 .0 + ring2.1 .0)
                    }
                }
            }
        }
    }
    return (min_cost, max_cost);
}

fn main() {
    let input_file = std::fs::read_to_string(
        std::path::PathBuf::from(std::env::var("FILE_DIR").unwrap())
            .parent()
            .unwrap()
            .join(".inputs/day21.txt"),
    )
    .expect("File Not Found!");

    let (_part1ans, _part2ans): (i32, i32) = part1n2(&input_file);
    println!("Day 21\nPart 1: {}\nPart 2: {}", _part1ans, _part2ans);
}
