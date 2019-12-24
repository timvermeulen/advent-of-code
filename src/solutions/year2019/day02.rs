use super::*;
use intcode::prelude::*;

fn run(memory: &[i64], noun: i64, verb: i64) -> i64 {
    let mut memory = memory.to_owned();
    memory[1] = noun;
    memory[2] = verb;
    let mut computer = Computer::new(memory);
    computer.step();
    computer.memory[0]
}

fn part1(memory: &[i64]) -> i64 {
    run(memory, 12, 2)
}

fn part2(memory: &[i64]) -> i64 {
    for noun in 0..100 {
        for verb in 0..100 {
            if run(memory, noun, verb) == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!()
}

pub fn solve(input: &str) -> (i64, i64) {
    let memory = intcode::parser().parse_to_end(&input).unwrap();
    (part1(&memory), part2(&memory))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 2).await?;
    let memory = intcode::parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&memory), 3_409_710);
    assert_eq!(part2(&memory), 7912);
    Ok(())
}
