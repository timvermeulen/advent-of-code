use super::{Grid, Pos};

pub fn solve(input: &str) -> (usize, usize) {
    let mut grid = Grid::new(false);
    for (line, y) in input.lines().zip(0..) {
        for (b, x) in line.bytes().zip(0..) {
            let pos = Pos { x, y };
            if b == b'#' {
                grid[pos] = true;
            }
        }
    }

    (part1(grid.clone()), part2(grid))
}

fn part1(mut grid: Grid<bool>) -> usize {
    for _ in 0..100 {
        let mut new = Grid::new(false);
        for x in 0..100 {
            for y in 0..100 {
                let pos = Pos { x, y };
                let count = pos.diag_neighbors().filter(|&n| grid[n]).count();
                if count == 3 || (count == 2 && grid[pos]) {
                    new[pos] = true;
                }
            }
        }
        grid = new;
    }

    grid.iter().filter(|&(_, &x)| x).count()
}

fn part2(mut grid: Grid<bool>) -> usize {
    for _ in 0..100 {
        let mut new = Grid::new(false);
        for x in 0..100 {
            for y in 0..100 {
                let pos = Pos { x, y };
                let count = pos.diag_neighbors().filter(|&n| grid[n]).count();
                if count == 3 || (count == 2 && grid[pos]) {
                    new[pos] = true;
                }
            }
        }
        new[Pos { x: 0, y: 0 }] = true;
        new[Pos { x: 0, y: 99 }] = true;
        new[Pos { x: 99, y: 0 }] = true;
        new[Pos { x: 99, y: 99 }] = true;
        grid = new;
    }

    grid.iter().filter(|&(_, &x)| x).count()
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 18).await?;
    assert_eq!(solve(&input), (1061, 1006));
    Ok(())
}
