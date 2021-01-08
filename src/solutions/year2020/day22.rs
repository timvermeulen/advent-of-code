use std::collections::VecDeque;

fn fight1(p1: impl Iterator<Item = u32>, p2: impl Iterator<Item = u32>) -> (bool, VecDeque<u32>) {
    let mut p1: VecDeque<_> = p1.collect();
    let mut p2: VecDeque<_> = p2.collect();

    while !p1.is_empty() && !p2.is_empty() {
        let x = p1.pop_front().unwrap();
        let y = p2.pop_front().unwrap();

        if x > y {
            p1.push_back(x);
            p1.push_back(y);
        } else {
            p2.push_back(y);
            p2.push_back(x);
        }
    }

    if p1.is_empty() {
        (false, p2)
    } else {
        (true, p1)
    }
}

fn fight2(p1: impl Iterator<Item = u32>, p2: impl Iterator<Item = u32>) -> (bool, VecDeque<u32>) {
    let mut p1: VecDeque<_> = p1.collect();
    let mut p2: VecDeque<_> = p2.collect();

    let mut seen = fxhash::FxHashSet::default();

    while !p1.is_empty() && !p2.is_empty() {
        let mut array = [0; 51];

        for (i, &n) in p1.iter().enumerate() {
            array[i] = n as u8;
        }

        for (i, &n) in p2.iter().enumerate() {
            array[i + p1.len() + 1] = n as u8;
        }

        if !seen.insert(array) {
            return (true, p1);
        }

        let x = p1.pop_front().unwrap();
        let y = p2.pop_front().unwrap();

        let p1_win = if x as usize <= p1.len() && y as usize <= p2.len() {
            let (p1_win, _) = fight2(
                p1.iter().copied().take(x as usize),
                p2.iter().copied().take(y as usize),
            );
            p1_win
        } else {
            x > y
        };

        if p1_win {
            p1.push_back(x);
            p1.push_back(y);
        } else {
            p2.push_back(y);
            p2.push_back(x);
        }
    }

    if p1.is_empty() {
        (false, p2)
    } else {
        (true, p1)
    }
}

fn score(deck: &VecDeque<u32>) -> u32 {
    (1..).zip(deck.iter().rev()).map(|(i, n)| i * n).sum()
}

pub fn solve(input: &str) -> (u32, u32) {
    let all: Vec<u32> = input.lines().filter_map(|l| l.parse().ok()).collect();
    let i = all.len() / 2;
    let (_, deck1) = fight1(all[..i].iter().copied(), all[i..].iter().copied());
    let (_, deck2) = fight2(all[..i].iter().copied(), all[i..].iter().copied());

    (score(&deck1), score(&deck2))
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2020, 22).await?;
    assert_eq!(solve(&input), (32_102, 34_173));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2020, 22)).unwrap();
        b.iter(|| solve(&input));
    }
}
