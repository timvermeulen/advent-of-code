use super::*;

fn helper(slice: &[u32], target: u32) -> Option<u32> {
    let mut i = 0;
    let mut j = slice.partition_point(|&x| x < target - slice[i]) - 1;

    while i < j {
        let sum = slice[i] + slice[j];

        match sum.cmp(&target) {
            Ordering::Less => i += 1,
            Ordering::Equal => return Some(slice[i] * slice[j]),
            Ordering::Greater => j -= 1,
        }
    }

    None
}

fn part1(entries: &[u32]) -> u32 {
    helper(entries, 2020).unwrap()
}

fn part2(entries: &[u32]) -> u32 {
    for i in 0.. {
        let x = entries[i];
        if let Some(product) = helper(&entries[(i + 1)..], 2020 - x) {
            return x * product;
        }
    }

    unreachable!()
}

pub fn solve(input: &str) -> (u32, u32) {
    let mut entries: Vec<_> = ascii_split(input, b'\n')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    entries.sort_unstable();
    (part1(&entries), part2(&entries))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2020, 1).await?;
    assert_eq!(solve(&input), (876_459, 116_168_640));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2020, 1)).unwrap();
        b.iter(|| solve(&input));
    }
}
