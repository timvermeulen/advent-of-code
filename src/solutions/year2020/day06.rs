#[cfg(test)]
use super::*;

pub fn solve(input: &str) -> (u32, u32) {
    const NONE: u32 = 0;
    const ALL: u32 = (1 << 27) - 2;
    const MASK: u8 = (1 << 5) - 1;

    let mut part1 = 0;
    let mut part2 = 0;

    let mut union = NONE;
    let mut intersection = ALL;

    let mut bits = NONE;
    let mut was_newline = false;

    for &byte in input.as_bytes() {
        if byte == b'\n' {
            if was_newline {
                part1 += union.count_ones();
                union = NONE;

                part2 += intersection.count_ones();
                intersection = ALL;

                bits = NONE;
            } else {
                union |= bits;
                intersection &= bits;
                bits = NONE;
            }

            was_newline = true;
        } else {
            bits |= 1 << (byte & MASK);
            was_newline = false;
        }
    }

    union |= bits;
    intersection &= bits;

    part1 += union.count_ones();
    part2 += intersection.count_ones();

    (part1, part2)
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2020, 6).await?;
    assert_eq!(solve(&input), (6726, 3316));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2020, 6)).unwrap();
        b.iter(|| solve(&input));
    }
}
