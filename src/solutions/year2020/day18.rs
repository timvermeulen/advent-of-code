use super::*;

#[derive(Copy, Clone)]
struct State {
    part1: u64,
    part1_adding: bool,
    part2: u64,
    part2_sum: u64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            part1: 0,
            part1_adding: true,
            part2: 1,
            part2_sum: 0,
        }
    }
}

impl State {
    fn process(&mut self, part1: u64, part2: u64) {
        if self.part1_adding {
            self.part1 += part1;
        } else {
            self.part1 *= part1;
        }
        self.part2_sum += part2;
    }

    fn add(&mut self) {
        self.part1_adding = true;
    }

    fn multiply(&mut self) {
        self.part1_adding = false;
        self.part2 *= self.part2_sum;
        self.part2_sum = 0;
    }

    fn finish(mut self) -> (u64, u64) {
        self.part2 *= self.part2_sum;
        (self.part1, self.part2)
    }
}

pub fn solve(input: &str) -> (u64, u64) {
    let mut part1 = 0;
    let mut part2 = 0;

    let mut array = [State::default(); 3];
    let mut current = 0;

    for byte in input.as_bytes() {
        match byte {
            b' ' => {}
            b'(' => {
                current += 1;
                array[current] = Default::default();
            }
            b')' => {
                let (a, b) = array[current].finish();
                current -= 1;
                array[current].process(a, b);
            }
            b'+' => array[current].add(),
            b'*' => array[current].multiply(),
            b'\n' => {
                let (a, b) = mem::take(&mut array[current]).finish();
                part1 += a;
                part2 += b;
            }
            _ => {
                let n = (byte - b'0') as u64;
                array[current].process(n, n);
            }
        }
    }

    let (a, b) = array[0].finish();
    part1 += a;
    part2 += b;

    (part1, part2)
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2020, 18).await?;
    assert_eq!(solve(&input), (23_507_031_841_020, 218_621_700_997_826));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2020, 18)).unwrap();
        b.iter(|| solve(&input));
    }
}
