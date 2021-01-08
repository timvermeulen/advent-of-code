use itertools::Itertools;
use std::cmp;

struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

const WEAPONS: &[Item] = &[
    Item {
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        cost: 74,
        damage: 8,
        armor: 0,
    },
];

const ARMOR: &[Item] = &[
    Item {
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Item {
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Item {
        cost: 102,
        damage: 0,
        armor: 5,
    },
];

const RINGS: &[Item] = &[
    Item {
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item {
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item {
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item {
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

pub fn solve(input: &str) {
    let mut iter = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok());

    let boss_hit_points = iter.next().unwrap();
    let boss_damage = iter.next().unwrap();
    let boss_armor = iter.next().unwrap();

    let mut min_cost = i32::MAX;
    let mut max_cost = i32::MIN;

    for weapon in WEAPONS {
        for armor in ARMOR {
            for (ring1, ring2) in RINGS.iter().tuple_combinations() {
                let stuff = [weapon, armor, ring1, ring2];
                let cost: i32 = stuff.iter().map(|i| i.cost).sum();

                let damage: i32 = stuff.iter().map(|i| i.damage).sum();
                let armor: i32 = stuff.iter().map(|i| i.armor).sum();

                if fight(damage, armor, boss_hit_points, boss_damage, boss_armor) {
                    min_cost = cmp::min(min_cost, cost);
                } else {
                    max_cost = cmp::max(max_cost, cost);
                }
            }
        }
    }

    println!("part 1: {}", min_cost);
    println!("part 2: {}", max_cost);
}

fn fight(
    player_damage: i32,
    player_armor: i32,
    boss_hit_points: i32,
    boss_damage: i32,
    boss_armor: i32,
) -> bool {
    let mut player = 100;
    let mut boss = boss_hit_points;

    loop {
        boss -= cmp::max(1, player_damage - boss_armor);
        if boss <= 0 {
            return true;
        }
        player -= cmp::max(1, boss_damage - player_armor);
        if player <= 0 {
            return false;
        }
    }
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 21).await?;
    assert_eq!(solve(&input), ());
    Ok(())
}
