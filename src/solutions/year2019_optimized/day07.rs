use super::*;
use intcode::prelude::*;
use permutohedron::Heap;

pub fn solve(input: &str) -> (i64, i64) {
    let memory = intcode::parser().parse_to_end(input).unwrap();
    let mut computer = Computer::new(memory);
    computer.step_with(0);

    let mut part1 = [(0, 0); 5];
    let mut part2 = [[0; 10]; 5];

    for i in 0..5 {
        let output = computer.step_with(128).unwrap();
        part1[i] = (output / 128, output % 128);
        computer.step();
    }

    for i in 0..5 {
        for j in 0..10 {
            let output = computer.step_with(0).unwrap();
            part2[i][j] = output;
        }
        computer.step();
    }

    let mut part1_max = 0;
    let mut heap = Heap::new(&mut part1);
    while let Some(p) = heap.next_permutation() {
        let result = p.iter().fold(0, |n, &(a, b)| a * n + b);
        part1_max = cmp::max(part1_max, result);
    }

    let mut part2_max = 0;
    let mut heap = Heap::new(&mut part2);
    while let Some(p) = heap.next_permutation() {
        let result = (0..10).fold(0, |signal, i| {
            p.iter().fold(signal, |signal, x| match unsafe { *x.get_unchecked(i) } {
                1 => signal + 1,
                2 => signal + 2,
                _ => signal * 2,
            })
        });
        part2_max = cmp::max(part1_max, result);
    }

    (part1_max, part2_max)
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 7).await?;
    assert_eq!(solve(&input), (212_460, 21_844_737));
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
        b.iter(|| solve(&input));
    }
}
