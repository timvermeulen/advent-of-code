use super::*;

fn parse(input: &str) -> Vec<u32> {
    parser::u32().collect_sep_by(token('\n')).parse_to_end(input).unwrap()
}

fn part1(modules: &[u32]) -> u32 {
    modules.iter().map(|m| m / 3 - 2).sum()
}

fn part2(modules: &[u32]) -> u32 {
    fn fuel(weight: u32) -> u32 {
        if weight < 9 {
            0
        } else {
            let f = weight / 3 - 2;
            f + fuel(f)
        }
    }
    modules.iter().map(|&m| fuel(m)).sum()
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 1).await?;
    let modules = parse(&input);
    assert_eq!(part1(&modules), 3_381_405);
    assert_eq!(part2(&modules), 5_069_241);
    Ok(())
}
