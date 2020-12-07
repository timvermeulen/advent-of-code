use super::*;

use permutohedron::Heap;

fn parse<'a>(input: &str) -> Vec<Vec<u32>> {
    let no_digit = satisfy(|c: char| !c.is_digit(10));
    let line = no_digit
        .skip_many()
        .followed_by(parser::u32())
        .map(|(_, n)| n);
    let lines = line.collect_sep_by(token('\n'));
    let distances: Vec<_> = lines.parse_to_end(input).unwrap();
    let n = reverse_triangle(distances.len()).unwrap();
    let m = n + 1;

    let mut grid = vec![vec![0; m]; m];
    let mut c = 0;
    for i in 0..n {
        for j in i..n {
            grid[i][j + 1] = distances[c];
            grid[j + 1][i] = distances[c];
            c += 1;
        }
    }
    grid
}

fn reverse_triangle(n: usize) -> Option<usize> {
    let mut k = 0;

    for i in 1.. {
        k += i;

        if k > n {
            return None;
        } else if k == n {
            return Some(i);
        }
    }

    unreachable!()
}

fn part1(grid: &[Vec<u32>]) -> u32 {
    let mut vec = (0..grid.len()).collect::<Vec<_>>();
    let mut heap = Heap::new(&mut vec);
    let mut min = u32::max_value();

    while let Some(permutation) = heap.next_permutation() {
        let sum: u32 = permutation.windows(2).map(|s| grid[s[0]][s[1]]).sum();
        min = min.min(sum);
    }

    min
}

fn part2(grid: &[Vec<u32>]) -> u32 {
    let mut vec = (0..grid.len()).collect::<Vec<_>>();
    let mut heap = Heap::new(&mut vec);
    let mut max = u32::min_value();

    while let Some(permutation) = heap.next_permutation() {
        let sum: u32 = permutation.windows(2).map(|s| grid[s[0]][s[1]]).sum();
        max = max.max(sum);
    }

    max
}

pub fn solve(input: &str) -> (u32, u32) {
    let grid = parse(&input);
    (part1(&grid), part2(&grid))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 9).await?;
    let grid = parse(&input);
    assert_eq!(part1(&grid), 207);
    assert_eq!(part2(&grid), 804);
    Ok(())
}
