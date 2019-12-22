use super::*;
use intcode::prelude::*;

fn run(mut memory: Vec<i64>, noun: i64, verb: i64) -> i64 {
    memory[1] = noun;
    memory[2] = verb;
    let mut computer = Computer::new(memory);
    computer.step();
    computer.memory[0]
}

fn solve(input: &str) -> (i64, i64) {
    let memory: Vec<i64> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let base = run(memory.clone(), 0, 0);
    let part1 = run(memory, 12, 2);
    let delta = (part1 - 2 - base) / 12;
    let z = 19_690_720 - base;
    (part1, 100 * z / delta + z % delta)
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 2).await?;
    assert_eq!(solve(&input), (3_409_710, 7912));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 2)).unwrap();
        b.iter(|| solve(&input));
    }
}
