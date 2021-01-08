use super::*;

fn part1(nums: &[u64]) -> u64 {
    unsafe {
        for (i, (j, n)) in nums.iter().copied().enumerate().skip(25).enumerate() {
            let slice = nums.get_unchecked(i..j);
            if slice
                .iter()
                .enumerate()
                .all(|(i, &k)| k > n || !slice.get_unchecked((i + 1)..).contains(&(n - k)))
            {
                return n;
            }
        }

        std::hint::unreachable_unchecked()
    }
}

fn part2(nums: &[u64], target: u64) -> u64 {
    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;

    loop {
        match sum.cmp(&target) {
            Ordering::Less => {
                sum += nums[j];
                j += 1;
            }
            Ordering::Equal => {
                let s = &nums[i..j];
                return s.iter().copied().min().unwrap() + s.iter().copied().max().unwrap();
            }
            Ordering::Greater => {
                sum -= nums[i];
                i += 1;
            }
        }
    }
}

fn parse(input: &str) -> Vec<u64> {
    ascii_split(input, b'\n')
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn solve(input: &str) -> (u64, u64) {
    let nums = parse(input);
    let x = part1(&nums);
    (x, part2(&nums, x))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2020, 9).await?;
    assert_eq!(solve(&input), (23278925, 4011064));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2020, 9)).unwrap();
        b.iter(|| solve(&input));
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2020, 9)).unwrap();
        let nums = parse(&input);
        b.iter(|| part1(&nums));
    }
}
