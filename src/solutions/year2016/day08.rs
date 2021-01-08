use super::*;

#[derive(Copy, Clone)]
enum State {
    On,
    Off,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::On => write!(f, "#"),
            Self::Off => write!(f, "."),
        }
    }
}

#[derive(Debug)]
enum Operation {
    Rect { width: i32, height: i32 },
    RotateRow { y: i32, by: usize },
    RotateColumn { x: i32, by: usize },
}

fn rect<'a>() -> impl Parser<&'a str, Output = Operation> {
    chain((string("rect "), parser::i32(), token('x'), parser::i32()))
        .map(|(_, width, _, height)| Operation::Rect { width, height })
}

fn rotate_row<'a>() -> impl Parser<&'a str, Output = Operation> {
    chain((
        string("rotate row y="),
        parser::i32(),
        string(" by "),
        parser::usize(),
    ))
    .map(|(_, y, _, by)| Operation::RotateRow { y, by })
}

fn rotate_column<'a>() -> impl Parser<&'a str, Output = Operation> {
    chain((
        string("rotate column x="),
        parser::i32(),
        string(" by "),
        parser::usize(),
    ))
    .map(|(_, x, _, by)| Operation::RotateColumn { x, by })
}

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<Operation>> {
    choice((rect().attempt(), rotate_row().attempt(), rotate_column())).collect_sep_by(newline())
}

pub fn solve(input: &str) {
    const WIDTH: i32 = 50;
    const HEIGHT: i32 = 6;

    let mut grid = Grid::new(State::Off);

    for op in parser().parse_to_end(input).unwrap() {
        match op {
            Operation::Rect { width, height } => {
                for x in 0..width {
                    for y in 0..height {
                        grid[Pos { x, y }] = State::On;
                    }
                }
            }
            Operation::RotateRow { y, by } => {
                let mut vec: Vec<_> = (0..WIDTH).map(|x| grid[Pos { x, y }]).collect();
                vec.rotate_right(by);
                (0..WIDTH)
                    .zip(vec)
                    .for_each(|(x, state)| grid[Pos { x, y }] = state);
            }
            Operation::RotateColumn { x, by } => {
                let mut vec: Vec<_> = (0..HEIGHT).map(|y| grid[Pos { x, y }]).collect();
                vec.rotate_right(by);
                (0..HEIGHT)
                    .zip(vec)
                    .for_each(|(y, state)| grid[Pos { x, y }] = state);
            }
        }
    }

    println!(
        "{}",
        grid.iter()
            .filter(|&(_, &state)| matches!(state, State::On))
            .count()
    );
    println!("{:?}", grid);
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 8).await?;
    solve(&input);
    Ok(())
}
