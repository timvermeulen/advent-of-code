use super::*;
use intcode::prelude::*;

fn part1(memory: &[i64]) -> i64 {
    run(memory, 1)
}

fn part2(memory: &[i64]) -> i64 {
    run(memory, 2)
}

fn run(memory: &[i64], input: i64) -> i64 {
    let mut comp = Computer::new(memory.to_owned());
    comp.step_with(input).unwrap()
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 9).await?;
    let memory = intcode::parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&memory), 2_457_252_183);
    assert_eq!(part2(&memory), 70_634);
    Ok(())
}
