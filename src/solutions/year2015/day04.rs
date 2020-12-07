use crypto::{digest::Digest, md5};

fn find_hash(key: &str, lower_bound: usize, mut condition: impl FnMut([u8; 16]) -> bool) -> usize {
    let mut hasher = md5::Md5::new();

    let mut hash = |n: usize| -> bool {
        hasher.input_str(key);
        hasher.input(n.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);
        hasher.reset();

        condition(output)
    };

    (lower_bound..).find(|&n| hash(n)).unwrap()
}

fn part1(key: &str) -> usize {
    find_hash(key, 1, |output| {
        output[0] == 0 && output[1] == 0 && output[2] >> 4 == 0
    })
}

fn part2(key: &str, lower: usize) -> usize {
    find_hash(key, lower, |output| {
        output[0] == 0 && output[1] == 0 && output[2] == 0
    })
}

pub fn solve(input: &str) -> (usize, usize) {
    let part1 = part1(input);
    (part1, part2(input, part1))
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 4).await?;
    let part1 = part1(&input);
    assert_eq!(part1, 254_575);
    assert_eq!(part2(&input, part1), 1_038_736);
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2015, 4)).unwrap();
        b.iter(|| part1(&input));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2015, 4)).unwrap();
        b.iter(|| part2(&input, 254_575));
    }
}
