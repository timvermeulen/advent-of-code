use super::*;

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<i64>> {
    parser::i64().collect_sep_by(comma())
}

fn run(mut memory: Vec<i64>, noun: i64, verb: i64) -> i64 {
    memory[1] = noun;
    memory[2] = verb;
    let mut computer = intcode::Computer::new(memory);
    computer.run();
    computer.memory[0]
}

fn part1(memory: Vec<i64>) -> i64 {
    run(memory, 12, 2)
}

fn part2(memory: Vec<i64>) -> i64 {
    for noun in 0..100 {
        for verb in 0..100 {
            if run(memory.clone(), noun, verb) == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!()
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 2).await?;
    let memory = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(memory.clone()), 3_409_710);
    assert_eq!(part2(memory), 7912);
    Ok(())
}
