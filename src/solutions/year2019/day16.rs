use super::*;

fn parse(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn part2(digits: &[u32]) -> u32 {
    let offset = digits[..7].iter().fold(0, |n, &d| 10 * n + d) as usize;
    let suffix_len = digits.len() * 10_000 - offset;

    let mut suffix: Vec<_> = digits.iter().copied().rev().cycle().take(suffix_len).collect();

    for _ in 0..100 {
        let mut prev = suffix[0];
        for x in &mut suffix[1..] {
            *x += prev;
            *x %= 10;
            prev = *x;
        }
    }

    suffix.iter().rev().take(8).fold(0, |n, &d| 10 * n + d)
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 16).await?;
    let digits = parse(&input);
    assert_eq!(part2(&digits), 52825021);
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 16)).unwrap();
        b.iter(|| {
            let digits = parse(&input);
            (part1(&digits), part2(&digits))
        });
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 16)).unwrap();
        b.iter(|| {
            let digits = parse(&input);
            part2(&digits)
        });
    }
}
