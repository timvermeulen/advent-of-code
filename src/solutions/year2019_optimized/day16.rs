use super::bits;
use std::{
    cmp,
    ops::{Index, IndexMut},
};

const NUM_PHASES: usize = 100;
const N: usize = 650;

fn part1(mut digits: [u32; N]) -> u32 {
    let mut partial_sums = [0; N + 1];

    for _ in 0..NUM_PHASES {
        for i in 0..N {
            partial_sums[i + 1] = partial_sums[i] + digits[i];
        }

        for (i, digit) in digits.iter_mut().enumerate() {
            let chain_len = i + 1;
            let neg_offset = 2 * chain_len;
            let cycle_len = 2 * neg_offset;

            let positive: u32 = (i..N)
                .step_by(cycle_len)
                .map(|i| partial_sums[cmp::min(N, i + chain_len)] - partial_sums[i])
                .sum();

            let negative: u32 = (i + neg_offset..N)
                .step_by(cycle_len)
                .map(|i| partial_sums[cmp::min(N, i + chain_len)] - partial_sums[i])
                .sum();

            *digit = (positive as i32 - negative as i32).abs() as u32 % 10;
        }
    }

    digits[..8].iter().fold(0, |n, &d| 10 * n + d)
}

// see https://github.com/pengi/advent_of_code/blob/master/2019/non_excel/day16p2.py
fn part2(digits: [u32; N]) -> u32 {
    let offset = digits[..7].iter().fold(0, |n, &d| 10 * n + d) as usize;
    let suffix_len = N * 10_000 - offset;
    let steps = suffix_len - 8;
    let batches = steps / N;

    let mut transition = Transition::new(&digits);
    let mut phases = Phases::zero();

    for bit in bits(batches) {
        if bit {
            phases = transition.apply(phases);
        }
        transition.double();
    }

    let digit_at = |i: usize| digits[N - 1 - i % N];

    for i in batches * N..steps {
        phases.step(digit_at(i));
    }

    let mut answer = 0;
    let mut n = 1;
    for i in steps..suffix_len {
        phases.step(digit_at(i));
        answer += n * phases[99];
        n *= 10;
    }
    answer
}

#[derive(Copy, Clone)]
struct Phases([u32; NUM_PHASES]);

impl Phases {
    fn zero() -> Self {
        Self([0; NUM_PHASES])
    }

    fn one() -> Self {
        let mut p = Self::zero();
        p[0] = 1;
        p
    }

    fn step(&mut self, start: u32) {
        let mut sum = start;
        for i in 0..NUM_PHASES {
            let phase = &mut self[i];
            *phase += sum;
            *phase %= 10;
            sum = *phase;
        }
    }
}

impl Index<usize> for Phases {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Phases {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

struct Transition {
    base_affect: Phases,
    phase_affect: Phases,
}

impl Transition {
    fn new(digits: &[u32; N]) -> Self {
        let mut base_affect = Phases::zero();
        for &digit in digits.iter().rev() {
            base_affect.step(digit);
        }

        let mut phase_affect = Phases::one();
        for _ in 0..digits.len() {
            phase_affect.step(0);
        }

        Self {
            base_affect,
            phase_affect,
        }
    }

    fn apply(&self, phases: Phases) -> Phases {
        self.apply_with_base(phases, self.base_affect)
    }

    fn apply_with_zero(&self, phases: Phases) -> Phases {
        self.apply_with_base(phases, Phases::zero())
    }

    fn apply_with_base(&self, phases: Phases, mut base: Phases) -> Phases {
        for i in 0..NUM_PHASES {
            for j in i..NUM_PHASES {
                base[j] += phases[i] * self.phase_affect[j - i];
                base[j] %= 10;
            }
        }
        base
    }

    fn double(&mut self) {
        self.base_affect = self.apply(self.apply(Phases::zero()));
        self.phase_affect = self.apply_with_zero(self.apply_with_zero(Phases::one()));
    }
}

pub fn solve(input: &str) -> (u32, u32) {
    let mut digits = [0; N];
    for i in 0..N {
        digits[i] = (input.as_bytes()[i] - b'0') as u32;
    }
    (part1(digits), part2(digits))
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 16).await?;
    assert_eq!(solve(&input), (68_764_632, 52_825_021));
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
        b.iter(|| solve(&input));
    }
}
