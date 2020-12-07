use super::*;

pub fn solve(input: &str) -> (u32, u32) {
    let mut min = u32::max_value();
    let mut max = u32::min_value();
    let mut sum = 0;

    for pass in input.as_bytes().chunks(11) {
        let id = unsafe { pass.get_unchecked(..10) }
            .iter()
            .map(|&b| (!b >> 2) & 1)
            .fold(0, |a, b| 2 * a + b as u32);

        min = cmp::min(min, id);
        max = cmp::max(max, id);
        sum += id;
    }

    (max, (max - min + 1) * (min + max) / 2 - sum)
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2020, 5).await?;
    assert_eq!(solve(&input), (930, 515));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2020, 5)).unwrap();
        b.iter(|| solve(&input));
    }
}
