use super::*;
use pathfinding::prelude::*;

fn part1(n: i32) -> i32 {
    let is_wall = |Pos { x, y }| {
        let sum = (x * x + 3 * x + 2 * x * y + y + y * y) + n;
        sum.count_ones() % 2 == 1
    };

    let start = Pos { x: 1, y: 1 };
    let goal = Pos { x: 31, y: 39 };

    let mut seen = HashSet::new();
    let mut current = vec![start];
    let mut next = vec![];

    for distance in 0.. {
        for p in current.drain(..) {
            if p == goal {
                return distance;
            }
            for n in p.non_neg_neighbors().filter(|&n| !is_wall(n)) {
                if seen.insert(n) {
                    next.push(n);
                }
            }
        }

        mem::swap(&mut current, &mut next);
    }

    unreachable!()
}

fn part2(n: i32) -> usize {
    let is_wall = |Pos { x, y }| {
        let sum = (x * x + 3 * x + 2 * x * y + y + y * y) + n;
        sum.count_ones() % 2 == 1
    };

    let start = Pos { x: 1, y: 1 };

    let mut seen = HashSet::new();
    let mut current = vec![start];
    let mut next = vec![];

    for distance in 0..50 {
        for p in current.drain(..) {
            for n in p.non_neg_neighbors().filter(|&n| !is_wall(n)) {
                if seen.insert(n) {
                    next.push(n);
                }
            }
        }

        mem::swap(&mut current, &mut next);
    }

    seen.len()
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 13).await?;
    let input = input.parse().unwrap();
    assert_eq!(part1(input), 96);
    assert_eq!(part2(input), 141);
    Ok(())
}
