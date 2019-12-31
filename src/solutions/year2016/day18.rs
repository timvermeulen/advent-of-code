pub fn solve(input: &str) -> (u32, u32) {
    let mut traps = 0_u128;
    for (i, byte) in input.bytes().enumerate() {
        if byte == b'^' {
            traps |= 1 << i;
        }
    }
    let len = input.len() as u32;
    (safe_count(40, traps, len), safe_count(400_000, traps, len))
}

fn safe_count(rows: u32, mut traps: u128, len: u32) -> u32 {
    let mask = (1 << len) - 1;
    let mut trap_count = 0;

    for _ in 0..rows {
        trap_count += traps.count_ones();
        traps = mask & ((traps << 1) ^ (traps >> 1));
    }

    rows * len - trap_count
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 18).await?;
    assert_eq!(solve(&input), (1989, 19_999_894));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2016, 18)).unwrap();
        b.iter(|| solve(&input));
    }
}
