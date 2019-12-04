use super::*;

fn parse(input: &str) -> Range<u32> {
    chain((parser::u32(), token('-'), parser::u32()))
        .map(|(a, _, b)| a..b)
        .parse_to_end(input)
        .unwrap()
}

fn part1(range: Range<u32>) -> usize {
    range
        .filter(|&n| {
            let mut digits = || digits_len(n, 6);
            let pairs = || digits().zip(digits().skip(1));
            pairs().all(|(x, y)| x <= y) && pairs().any(|(x, y)| x == y)
        })
        .count()
}

fn part2(range: Range<u32>) -> usize {
    range
        .filter(|&n| {
            let digits = || digits_len(n, 6);
            let counts = digits().frequencies();
            digits().zip(digits().skip(1)).all(|(x, y)| x <= y) && counts.values().any(|&x| x == 2)
        })
        .count()
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 4).await?;
    let range = parse(&input);
    assert_eq!(part1(range.clone()), 1767);
    assert_eq!(part2(range), 1192);
    Ok(())
}
