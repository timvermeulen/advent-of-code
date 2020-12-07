use super::*;

enum Direction {
    Up,
    Down,
}

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<Direction>> {
    choice((
        token('(').map(|_| Direction::Up),
        token(')').map(|_| Direction::Down),
    ))
    .collect_many()
}

fn part1(directions: &[Direction]) -> i32 {
    directions
        .iter()
        .map(|d| match d {
            Direction::Up => 1,
            Direction::Down => -1,
        })
        .sum()
}

fn part2(directions: &[Direction]) -> usize {
    directions
        .iter()
        .scan(0, |sum, d| {
            match d {
                Direction::Up => *sum += 1,
                Direction::Down => *sum -= 1,
            }
            Some(*sum)
        })
        .position(|x| x == -1)
        .unwrap()
        + 1
}

pub fn solve(input: &str) -> (i32, usize) {
    let directions = parser().parse_to_end(&input).unwrap();
    (part1(&directions), part2(&directions))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 1).await?;
    let directions = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&directions), 138);
    assert_eq!(part2(&directions), 1771);
    Ok(())
}
