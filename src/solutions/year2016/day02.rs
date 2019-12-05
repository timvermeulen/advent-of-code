use super::*;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn offset1(&self, digit: char) -> char {
        match (self, digit) {
            (Direction::Right, '1') => '2',
            (Direction::Down, '1') => '4',
            (Direction::Right, '2') => '3',
            (Direction::Down, '2') => '5',
            (Direction::Left, '2') => '1',
            (Direction::Down, '3') => '6',
            (Direction::Left, '3') => '2',
            (Direction::Up, '4') => '1',
            (Direction::Right, '4') => '5',
            (Direction::Down, '4') => '7',
            (Direction::Up, '5') => '2',
            (Direction::Right, '5') => '6',
            (Direction::Down, '5') => '8',
            (Direction::Left, '5') => '4',
            (Direction::Up, '6') => '3',
            (Direction::Down, '6') => '9',
            (Direction::Left, '6') => '5',
            (Direction::Up, '7') => '4',
            (Direction::Right, '7') => '8',
            (Direction::Up, '8') => '5',
            (Direction::Right, '8') => '9',
            (Direction::Left, '8') => '7',
            (Direction::Up, '9') => '6',
            (Direction::Left, '9') => '8',
            (_, digit) => digit,
        }
    }

    fn offset2(&self, digit: char) -> char {
        match (self, digit) {
            (Direction::Down, '1') => '3',

            (Direction::Right, '2') => '3',
            (Direction::Down, '2') => '6',

            (Direction::Up, '3') => '1',
            (Direction::Right, '3') => '4',
            (Direction::Down, '3') => '7',
            (Direction::Left, '3') => '2',

            (Direction::Down, '4') => '8',
            (Direction::Left, '4') => '3',

            (Direction::Right, '5') => '6',

            (Direction::Up, '6') => '2',
            (Direction::Right, '6') => '7',
            (Direction::Down, '6') => 'A',
            (Direction::Left, '6') => '5',

            (Direction::Up, '7') => '3',
            (Direction::Right, '7') => '8',
            (Direction::Down, '7') => 'B',
            (Direction::Left, '7') => '6',

            (Direction::Up, '8') => '4',
            (Direction::Right, '8') => '9',
            (Direction::Down, '8') => 'C',
            (Direction::Left, '8') => '7',

            (Direction::Left, '9') => '8',

            (Direction::Up, 'A') => '6',
            (Direction::Right, 'A') => 'B',

            (Direction::Up, 'B') => '7',
            (Direction::Right, 'B') => 'C',
            (Direction::Down, 'B') => 'D',
            (Direction::Left, 'B') => 'A',

            (Direction::Up, 'C') => '8',
            (Direction::Left, 'C') => 'B',

            (Direction::Up, 'D') => 'B',

            (_, digit) => digit,
        }
    }
}

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<Vec<Direction>>> {
    let direction = choice((
        token('U').map(|_| Direction::Up),
        token('R').map(|_| Direction::Right),
        token('D').map(|_| Direction::Down),
        token('L').map(|_| Direction::Left),
    ));
    let digit = direction.collect_many1();
    let digits = digit.collect_sep_by(token('\n'));
    digits
}

fn part1(digits: &[Vec<Direction>]) -> String {
    let mut current = '5';
    digits
        .iter()
        .map(|digit| {
            current = digit.iter().fold(current, |c, d| d.offset1(c));
            current
        })
        .collect()
}

fn part2(digits: &[Vec<Direction>]) -> String {
    let mut current = '5';
    digits
        .iter()
        .map(|digit| {
            current = digit.iter().fold(current, |c, d| d.offset2(c));
            current
        })
        .collect()
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 2).await?;
    let directions = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&directions), "84452");
    assert_eq!(part2(&directions), "D65C3");
    Ok(())
}
