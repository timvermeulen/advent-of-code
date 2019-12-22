use super::*;
use intcode::prelude::*;

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
        let color = match bot.step_with(input) {
            Interrupt::Halt => return paint.iter().count(),
            Interrupt::WaitingForInput => unreachable!(),
            Interrupt::Output(x) => x,
        };
        paint[pos] = match color {
            0 => Color::Black,
            1 => Color::White,
            _ => unreachable!(),
        };
        match bot.step().unwrap() {
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
        if bot.step().is_halt() {
            return break;
        }
        let input = (paint[pos] == Color::White) as i64;
        paint[pos] = match bot.step_with(input).unwrap() {
            0 => Color::Black,
            1 => Color::White,
            _ => unreachable!(),
        };
        match bot.step().unwrap() {
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
    let memory = intcode::parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(memory.clone()), 1883);
    part2(memory);
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 11)).unwrap();
        b.iter(|| {
            let memory = intcode::parser().parse_to_end(&input).unwrap();
            (part1(memory.clone()), part2(memory))
        });
    }
}
