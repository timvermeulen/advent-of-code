use super::*;

fn parse(input: String) -> Vec<i32> {
    let number = token('+').optional().followed_by(parser::i32()).map(|(_, x)| x);
    number.collect_sep_by(token('\n')).parse_to_end(&input).unwrap()
}

fn part1(frequencies: &[i32]) -> i32 {
    frequencies.iter().sum()
}

fn part2(frequencies: &[i32]) -> i32 {
    let mut set = HashSet::new();
    set.insert(0);
    frequencies
        .iter()
        .cycle()
        .scan(0, |a, b| {
            *a += b;
            Some(*a)
        })
        .find(|&x| !set.insert(x))
        .unwrap()
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2018, 1).await?;
    let frequencies = parse(input);
    assert_eq!(part1(&frequencies), 516);
    assert_eq!(part2(&frequencies), 71892);
    Ok(())
}
