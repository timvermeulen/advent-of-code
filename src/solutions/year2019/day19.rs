use super::*;
use intcode::prelude::*;

fn is_pulled(memory: &[i64], x: i64, y: i64) -> bool {
    let mut computer = Computer::new(memory.to_owned());
    computer.step_with(x);
    computer.step_with(y).unwrap() == 1
}

fn part1(memory: &[i64]) -> u32 {
    let is_pulled = |x: i64, y: i64| is_pulled(memory, x, y);
    let mut area = 0;
    for x in 0..50 {
        for y in 0..50 {
            if is_pulled(x, y) {
                area += 1;
            }
        }
    }
    area
}

fn part2(memory: &[i64]) -> i64 {
    let is_pulled = |x: i64, y: i64| is_pulled(memory, x, y);
    let mut min_x = 0;
    let mut max_y = 98;
    loop {
        max_y += 1;
        while !is_pulled(min_x, max_y) {
            min_x += 1;
        }
        let max_x = min_x + 99;
        let min_y = max_y - 99;
        if min_y >= 0 && is_pulled(max_x, min_y) {
            return min_x * 10_000 + min_y;
        }
    }
}

pub fn solve(input: &str) -> (u32, i64) {
    let memory = intcode::parser().parse_to_end(&input).unwrap();
    (part1(&memory), part2(&memory))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 19).await?;
    let memory = intcode::parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&memory), 234);
    assert_eq!(part2(&memory), 9_290_812);
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 19)).unwrap();
        b.iter(|| {
            let memory = intcode::parser().parse_to_end(&input).unwrap();
            (part1(&memory), part2(&memory))
        });
    }
}
