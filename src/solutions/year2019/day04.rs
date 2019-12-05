use super::*;

fn parse(input: &str) -> Range<u32> {
    chain((parser::u32(), token('-'), parser::u32()))
        .map(|(a, _, b)| a..b)
        .parse_to_end(input)
        .unwrap()
}

fn digits(n: u32) -> [u32; 6] {
    [n / 100_000, n / 10_000 % 10, n / 1000 % 10, n / 100 % 10, n / 10 % 10, n % 10]
}

fn part1(range: Range<u32>) -> usize {
    range
        .filter(|&n| {
            let digits = digits(n);
            let pairs = || digits.windows(2).map(|w| [w[0], w[1]]);
            pairs().all(|[x, y]| x <= y) && pairs().any(|[x, y]| x == y)
        })
        .count()
}

fn part2(range: Range<u32>) -> usize {
    range
        .filter(|&n| {
            let digits = digits(n);
            let mut freq = [0; 10];
            digits.iter().for_each(|&d| freq[d as usize] += 1);
            freq.iter().any(|&f| f == 2) && digits.windows(2).all(|w| w[0] <= w[1])
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

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 4)).unwrap();
        let range = parse(&input);
        b.iter(|| part1(range.clone()));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 4)).unwrap();
        let range = parse(&input);
        b.iter(|| part2(range.clone()));
    }
}
