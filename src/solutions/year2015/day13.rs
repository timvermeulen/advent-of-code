use permutohedron::Heap;

fn part1(changes: [[i32; 8]; 8]) -> i32 {
    let mut order = [1, 2, 3, 4, 5, 6, 7];
    let mut heap = Heap::new(&mut order);
    let mut max = i32::min_value();
    while let Some(p) = heap.next_permutation() {
        let pair = |i: usize, j: usize| changes[i][j] + changes[j][i];
        let change: i32 =
            pair(0, p[0]) + pair(0, p[6]) + (0..6).map(|i| pair(p[i], p[i + 1])).sum::<i32>();
        if change > max {
            max = change;
        }
    }
    max
}

fn part2(changes: [[i32; 8]; 8]) -> i32 {
    let mut order = [0, 1, 2, 3, 4, 5, 6, 7];
    let mut heap = Heap::new(&mut order);
    let mut max = i32::min_value();
    while let Some(p) = heap.next_permutation() {
        let pair = |i: usize, j: usize| changes[i][j] + changes[j][i];
        let change: i32 = (0..7).map(|i| pair(p[i], p[i + 1])).sum::<i32>();
        if change > max {
            max = change;
        }
    }
    max
}

pub fn solve(input: &str) -> (i32, i32) {
    let mut changes = [[0; 8]; 8];
    let mut lines = input.lines();
    for i in 0..8 {
        for j in 0..8 {
            if i != j {
                let line = lines.next().unwrap();
                let mut words = line.split(' ');
                let sign = match words.nth(2).unwrap() {
                    "lose" => -1,
                    "gain" => 1,
                    _ => unreachable!(),
                };
                changes[i][j] = sign * words.next().unwrap().parse::<i32>().unwrap();
            }
        }
    }

    (part1(changes), part2(changes))
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 13).await?;
    assert_eq!(solve(&input), (709, 668));
    Ok(())
}
