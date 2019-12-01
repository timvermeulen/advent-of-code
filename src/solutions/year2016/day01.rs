use super::*;

#[derive(Copy, Clone)]
enum Side {
    Left,
    Right,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(self, side: Side) -> Direction {
        match (self, side) {
            (Direction::Up, Side::Left) | (Direction::Down, Side::Right) => Direction::Left,
            (Direction::Right, Side::Left) | (Direction::Left, Side::Right) => Direction::Up,
            (Direction::Down, Side::Left) | (Direction::Up, Side::Right) => Direction::Right,
            (Direction::Left, Side::Left) | (Direction::Right, Side::Right) => Direction::Down,
        }
    }
}

struct Instruction {
    side: Side,
    amount: i32,
}

fn parse(input: &str) -> Vec<Instruction> {
    let side = choice((token('L').map(|_| Side::Left), token('R').map(|_| Side::Right)));

    let instruction =
        chain((side, parser::i32())).map(|(side, amount)| Instruction { side, amount });

    instruction.collect_sep_by(string(", ")).parse_to_end(input).unwrap()
}

fn part1(instructions: &[Instruction]) -> i32 {
    let mut h = 0;
    let mut v = 0;
    let mut direction = Direction::Up;

    for instruction in instructions {
        direction = direction.turn(instruction.side);
        match direction {
            Direction::Up => v -= instruction.amount,
            Direction::Down => v += instruction.amount,
            Direction::Left => h -= instruction.amount,
            Direction::Right => h += instruction.amount,
        }
    }

    h.abs() + v.abs()
}

fn part2(instructions: &[Instruction]) -> i32 {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    let mut location = (0_i32, 0_i32);
    let mut direction = Direction::Up;

    for instruction in instructions {
        direction = direction.turn(instruction.side);

        for _ in 0..instruction.amount {
            match direction {
                Direction::Up => location.1 -= 1,
                Direction::Down => location.1 += 1,
                Direction::Left => location.0 -= 1,
                Direction::Right => location.0 += 1,
            }

            if !set.insert(location) {
                return location.0.abs() + location.1.abs();
            }
        }
    }

    unreachable!()
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 1).await?;
    let instructions = parse(&input);
    assert_eq!(part1(&instructions), 278);
    assert_eq!(part2(&instructions), 161);
    Ok(())
}
