use super::*;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<Direction>> {
    let direction = choice((
        token('^').map(|_| Direction::Up),
        token('>').map(|_| Direction::Right),
        token('v').map(|_| Direction::Down),
        token('<').map(|_| Direction::Left),
    ));
    direction.collect_many()
}

fn part1(directions: &[Direction]) -> usize {
    let mut location = (0, 0);
    let mut set = HashSet::new();
    set.insert(location);

    for direction in directions {
        match direction {
            Direction::Up => location.1 -= 1,
            Direction::Right => location.0 += 1,
            Direction::Down => location.1 += 1,
            Direction::Left => location.0 -= 1,
        }
        set.insert(location);
    }

    set.len()
}

fn part2(directions: &[Direction]) -> usize {
    let mut locations = [(0, 0); 2];
    let mut is_robot = false;
    let mut set = HashSet::new();
    set.insert((0, 0));

    for direction in directions {
        let (x, y) = &mut locations[is_robot as usize];
        is_robot = !is_robot;
        match direction {
            Direction::Up => *y -= 1,
            Direction::Right => *x += 1,
            Direction::Down => *y += 1,
            Direction::Left => *x -= 1,
        }
        set.insert((*x, *y));
    }

    set.len()
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 3).await?;
    let directions = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&directions), 2565);
    assert_eq!(part2(&directions), 2639);
    Ok(())
}
