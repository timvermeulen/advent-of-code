use super::*;

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<i32>> {
    parser::i32().collect_sep_by(comma())
}

fn run(data: Vec<i32>, input: i32) -> i32 {
    let mut output = None;
    intcode::State::new(data, || input, |o| output = Some(o)).run();
    output.unwrap()
}

fn part1(data: Vec<i32>) -> i32 {
    run(data, 1)
}

fn part2(data: Vec<i32>) -> i32 {
    run(data, 5)
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 5).await?;
    let data = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(data.clone()), 15_097_178);
    assert_eq!(part2(data), 1_558_663);
    Ok(())
}
