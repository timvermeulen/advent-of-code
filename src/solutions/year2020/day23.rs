fn run(digits: &[usize], size: usize, moves: usize) -> Vec<u32> {
    let mut lookup = vec![0_u32; size];

    for &[a, b] in digits.array_windows() {
        lookup[a] = b as u32;
    }

    let mut prev = *digits.last().unwrap();

    for x in digits.len()..size {
        lookup[prev] = x as u32;
        prev = x;
    }

    let mut current = *digits.first().unwrap() as u32;
    lookup[prev] = current;

    for _ in 0..moves {
        let a = lookup[current as usize];
        let b = lookup[a as usize];
        let c = lookup[b as usize];
        let d = lookup[c as usize];

        let mut target = if current == 0 {
            size as u32 - 1
        } else {
            current - 1
        };

        while [a, b, c].contains(&target) {
            target = if target == 0 {
                size as u32 - 1
            } else {
                target - 1
            };
        }

        lookup[current as usize] = d;
        current = d;

        lookup[c as usize] = lookup[target as usize];
        lookup[target as usize] = a;
    }

    lookup
}

fn part1(digits: &[usize]) -> u32 {
    let lookup = run(&digits, digits.len(), 100);

    let mut ans = 0;
    let mut x = 0;

    for _ in 0..8 {
        let next = lookup[x as usize];
        ans *= 10;
        ans += next + 1;
        x = next;
    }

    ans
}

fn part2(digits: &[usize]) -> usize {
    let lookup = run(&digits, 1_000_000, 10_000_000);
    let x = lookup[0];
    let y = lookup[x as usize];
    (x as usize + 1) * (y as usize + 1)
}

pub fn solve(input: &str) -> (u32, usize) {
    let digits: Vec<_> = input.bytes().map(|b| (b - b'1') as usize).collect();
    (part1(&digits), part2(&digits))
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2020, 23).await?;
    assert_eq!(solve(&input), (72_496_583, 41_785_843_847));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2020, 23)).unwrap();
        b.iter(|| solve(&input));
    }
}
