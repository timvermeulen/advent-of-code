use super::*;
use intcode::*;

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<i64>> {
    parser::i64().collect_sep_by(comma())
}

#[derive(Copy, Clone, PartialEq)]
enum Color {
    White,
    Black,
}

impl Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::White => write!(f, "{}", 'X'),
            Self::Black => write!(f, "{}", '.'),
        }
    }
}

fn part1(memory: Vec<i64>) -> usize {
    let mut bot = Computer::new(memory);
    let mut paint = Grid::new(Color::Black);
    let mut pos = Pos { x: 0, y: 0 };
    let mut dir = Dir::North;

    loop {
        let input = (paint[pos] == Color::White) as i64;
        let color = match bot.run_with(input) {
            State::Halt => return paint.iter().count(),
            State::WaitingForInput => unreachable!(),
            State::Output(x) => x,
        };
        paint[pos] = match color {
            0 => Color::Black,
            1 => Color::White,
            _ => unreachable!(),
        };
        match bot.run().unwrap() {
            0 => dir.turn_left(),
            1 => dir.turn_right(),
            _ => unreachable!(),
        }
        pos.move_to(dir);
    }
}

fn part2(memory: Vec<i64>) {
    let mut bot = Computer::new(memory);
    let mut paint = Grid::new(Color::Black);
    let mut pos = Pos { x: 0, y: 0 };
    let mut dir = Dir::North;
    paint[pos] = Color::White;

    loop {
        if bot.run().is_halt() {
            return break;
        }
        let input = (paint[pos] == Color::White) as i64;
        paint[pos] = match bot.run_with(input).unwrap() {
            0 => Color::Black,
            1 => Color::White,
            _ => unreachable!(),
        };
        match bot.run().unwrap() {
            0 => dir.turn_left(),
            1 => dir.turn_right(),
            _ => unreachable!(),
        }
        pos.move_to(dir);
    }

    println!("{:?}", paint);
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 11).await?;
    let memory = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(memory.clone()), 1883);
    part2(memory);
    Ok(())
}
