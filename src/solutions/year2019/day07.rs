use super::*;
use intcode::prelude::*;
use permutohedron::Heap;

fn part1(memory: &[i64]) -> i64 {
    let mut settings = [0, 1, 2, 3, 4];
    let mut heap = Heap::new(&mut settings);
    let iter = iter::from_fn(|| {
        let p = heap.next_permutation()?;
        Some([p[0], p[1], p[2], p[3], p[4]])
    });
    iter.map(|settings| run_part1(memory, settings)).max().unwrap()
}

fn run_part1(memory: &[i64], settings: [i64; 5]) -> i64 {
    let amp = || Computer::new(memory.to_owned());
    let amps = [amp(), amp(), amp(), amp(), amp()];
    iter!(amps)
        .zip(&settings)
        .fold(0, |input, (mut amp, &setting)| amp.step_with_iter(iter!([setting, input])).unwrap())
}

fn part2(memory: &[i64]) -> i64 {
    let mut settings = [5, 6, 7, 8, 9];
    let mut heap = Heap::new(&mut settings);
    let iter = iter::from_fn(|| {
        let p = heap.next_permutation()?;
        Some([p[0], p[1], p[2], p[3], p[4]])
    });
    iter.map(|settings| run_part2(memory, settings)).max().unwrap()
}

fn run_part2(memory: &[i64], settings: [i64; 5]) -> i64 {
    let amp = || Computer::new(memory.to_owned());
    let mut amps = [amp(), amp(), amp(), amp(), amp()];
    let mut settings = settings.iter().copied();

    let mut value = 0;
    loop {
        for amp in &mut amps {
            if let Some(setting) = settings.next() {
                amp.step_with(setting);
            }
            value = amp.step_with(value).output().unwrap_or(value);
        }
        if amps[4].is_halted() {
            return value;
        }
    }
}

pub fn solve(input: &str) -> (i64, i64) {
    let memory = intcode::parser().parse_to_end(&input).unwrap();
    (part1(&memory), part2(&memory))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 7).await?;
    let memory = intcode::parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&memory), 212_460);
    assert_eq!(part2(&memory), 21_844_737);
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 7)).unwrap();
        b.iter(|| {
            let memory = intcode::parser().parse_to_end(&input).unwrap();
            (part1(&memory), part2(&memory))
        });
    }
}
