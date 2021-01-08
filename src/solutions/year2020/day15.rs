#[cfg(test)]
use super::*;

fn run(input: &[u32], rounds: usize) -> u32 {
    let mut seen = vec![0; rounds];

    for (round, &n) in (1..input.len() as u32).zip(input) {
        seen[n as usize] = round;
    }

    (input.len()..rounds).fold(*input.last().unwrap(), |number, round| {
        let round = round as u32;
        let prev = std::mem::replace(unsafe { seen.get_unchecked_mut(number as usize) }, round);
        (prev != 0) as u32 * (round - prev)
    })
}

pub fn solve(input: &str) -> (u32, u32) {
    let input: Vec<u32> = input.split(',').map(|s| s.parse().unwrap()).collect();
    (run(&input, 2020), run(&input, 30_000_000))
}

pub fn solve_part1(input: &str) -> (u32, u32) {
    let input: Vec<u32> = input.split(',').map(|s| s.parse().unwrap()).collect();
    (run(&input, 2020), run(&input, 2020))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2020, 15).await?;
    assert_eq!(solve(&input), (257, 8546398));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2020, 15)).unwrap();
        b.iter(|| solve(&input));
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2020, 15)).unwrap();
        b.iter(|| solve_part1(&input));
    }
}
