use super::*;
use intcode::*;

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<i32>> {
    parser::i32().collect_sep_by(comma())
}

fn run(memory: Vec<i32>, input: i32) -> i32 {
    let mut i = None;
    let mut o = None;
    let mut computer = Computer::new(memory);
    loop {
        match computer.run_with_input(i.take()) {
            State::Halt => return o.unwrap(),
            State::WaitingForInput => i = Some(input),
            State::Output(output) => o = Some(output),
        }
    }
}

fn part1(memory: Vec<i32>) -> i32 {
    run(memory, 1)
}

fn part2(memory: Vec<i32>) -> i32 {
    run(memory, 5)
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 5).await?;
    let memory = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(memory.clone()), 15_097_178);
    assert_eq!(part2(memory), 1_558_663);
    Ok(())
}
