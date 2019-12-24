use super::*;

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<u32>> {
    parser::u32().collect_sep_by(token('\n'))
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

pub fn solve(input: &str) -> (u32, u32) {
    let modules = parser().parse_to_end(&input).unwrap();
    (part1(&modules), part2(&modules))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 1).await?;
    let modules = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&modules), 3_381_405);
    assert_eq!(part2(&modules), 5_069_241);
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 1)).unwrap();
        b.iter(|| {
            let modules = parser().parse_to_end(&input).unwrap();
            (part1(&modules), part2(&modules))
        });
    }
}
