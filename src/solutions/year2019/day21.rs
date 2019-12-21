use super::*;
use intcode::prelude::*;

const PART1: &str = "\
NOT D J
OR C J
AND A J
NOT J J
WALK
";

const PART2: &str = "\
NOT A J
NOT T T
AND B T
AND C T
NOT T T
AND D T
AND H T
OR T J
RUN
";

fn run(memory: Vec<i64>, input: &str) -> i64 {
    let mut droid = Computer::new(memory);
    droid.run_with_iter(input.bytes().map(|b| b as i64)).last().unwrap()
}

fn part1(memory: Vec<i64>) -> i64 {
    run(memory, PART1)
}

fn part2(memory: Vec<i64>) -> i64 {
    run(memory, PART2)
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 21).await?;
    let memory = intcode::parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(memory.clone()), 19_359_316);
    assert_eq!(part2(memory), 1_141_281_622);
    Ok(())
}
