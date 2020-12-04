use super::*;
use fast_intcode::*;

fn part1(memory: Vec<i64>) -> i64 {
    run(memory, 1)
}

fn part2(memory: Vec<i64>) -> i64 {
    run(memory, 2)
}

fn run(memory: Vec<i64>, input: i64) -> i64 {
    let mut comp = Computer::new(memory.to_owned());
    comp.step_with(input).unwrap()
}

pub fn solve(input: &str) -> (i64, i64) {
    let memory = intcode::parser().parse_to_end(&input).unwrap();
    (part1(memory.clone()), part2(memory))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 9).await?;
    let memory = intcode::parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(memory.clone()), 2_457_252_183);
    assert_eq!(part2(memory), 70_634);
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 9)).unwrap();
        b.iter(|| {
            let memory = intcode::parser().parse_to_end(&input).unwrap();
            (part1(memory.clone()), part2(memory))
        });
    }
}
