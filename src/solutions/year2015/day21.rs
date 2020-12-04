use std::cmp;

pub fn solve(input: &str) {}

fn fight(player_damage: i32, player_armor: i32, boss_damage: i32, boss_armor: i32) -> bool {
    let mut player = 100;
    let mut boss = 100;

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
