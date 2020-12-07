#[cfg(test)]
use super::*;

pub fn solve(input: &str) -> (u32, u32) {
    const WIDTH: usize = 31;

    #[derive(Default, Copy, Clone)]
    struct Slope {
        count: u32,
        x: usize,
    }

    impl Slope {
        fn apply(&mut self, k: usize, chunk: &[u8]) {
            self.count += (unsafe { *chunk.get_unchecked(self.x) } == b'#') as u32;
            self.x += k;
            self.x %= WIDTH;
        }
    }

    let mut slopes = [Slope::default(); 5];

    for (i, chunk) in input.as_bytes().chunks(WIDTH + 1).enumerate() {
        slopes[0].apply(1, chunk);
        slopes[1].apply(3, chunk);
        slopes[2].apply(5, chunk);
        slopes[3].apply(7, chunk);

        if i % 2 == 0 {
            slopes[4].apply(1, chunk);
        }
    }

    (slopes[1].count, slopes.iter().map(|p| p.count).product())
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2020, 3).await?;
    assert_eq!(solve(&input), (282, 958_815_792));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2020, 3)).unwrap();
        b.iter(|| solve(&input));
    }
}
