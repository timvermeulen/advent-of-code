use super::*;
use intcode::prelude::*;

fn run(memory: Vec<i64>, input: i64) -> i64 {
    let mut computer = Computer::new(memory);
    computer.run_with(input).last().unwrap()
}

fn part1(memory: Vec<i64>) -> i64 {
    run(memory, 1)
}

fn part2(memory: Vec<i64>) -> i64 {
    run(memory, 5)
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 5).await?;
    let memory = intcode::parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(memory.clone()), 15_097_178);
    assert_eq!(part2(memory), 1_558_663);
    Ok(())
}
