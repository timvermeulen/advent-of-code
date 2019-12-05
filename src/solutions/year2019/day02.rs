use super::*;

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<i32>> {
    parser::i32().collect_sep_by(comma())
}

fn run(mut data: Vec<i32>, noun: i32, verb: i32) -> i32 {
    data[1] = noun;
    data[2] = verb;
    let mut state = intcode::State::new(
        data,
        || panic!("unexpected input opcode"),
        |_| panic!("unexpected output opcode"),
    );
    state.run();
    state.data[0]
}

fn part1(data: Vec<i32>) -> i32 {
    run(data, 12, 2)
}

fn part2(data: Vec<i32>) -> i32 {
    for noun in 0..100 {
        for verb in 0..100 {
            if run(data.clone(), noun, verb) == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!()
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 2).await?;
    let data = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(data.clone()), 3_409_710);
    assert_eq!(part2(data), 7912);
    Ok(())
}
