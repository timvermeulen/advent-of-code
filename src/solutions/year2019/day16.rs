use super::*;

fn parse(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn part1(digits: &[u32]) -> u32 {
    let pattern =
        |i| [0, 1, 0, -1].iter().flat_map(move |&x| iter::repeat(x).take(i + 1)).cycle().skip(1);
    let mut vec = digits.to_owned();
    for _ in 0..100 {
        vec = (0..vec.len())
            .map(|i| {
                pattern(i).zip(&vec).map(|(a, &b)| a * b as i32).sum::<i32>().abs() as u32 % 10
            })
            .collect();
    }
    vec[..8].iter().fold(0, |n, &d| 10 * n + d)
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
    assert_eq!(part1(&digits), 68_764_632);
    assert_eq!(part2(&digits), 52_825_021);
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
