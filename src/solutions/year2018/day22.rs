use super::*;
use search_algs::*;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Type {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum State {
    Torch,
    Climbing,
    Neither,
}

impl State {
    fn works_for(self, t: Type) -> bool {
        match (self, t) {
            (Self::Torch, Type::Wet)
            | (Self::Climbing, Type::Narrow)
            | (Self::Neither, Type::Rocky) => false,
            _ => true,
        }
    }

    fn other_state_for(self, t: Type) -> Self {
        match (self, t) {
            (Self::Torch, Type::Rocky) | (Self::Neither, Type::Wet) => Self::Climbing,
            (Self::Torch, Type::Narrow) | (Self::Climbing, Type::Wet) => Self::Neither,
            (Self::Climbing, Type::Rocky) | (Self::Neither, Type::Narrow) => Self::Torch,
            _ => unreachable!(),
        }
    }
}

impl Type {
    fn new(erosion: i32) -> Self {
        match erosion % 3 {
            0 => Self::Rocky,
            1 => Self::Wet,
            2 => Self::Narrow,
            _ => unreachable!(),
        }
    }

    fn risk_level(self) -> i32 {
        match self {
            Self::Rocky => 0,
            Self::Wet => 1,
            Self::Narrow => 2,
        }
    }
}

fn erosion(
    depth: i32,
    target: Pos,
) -> Cache<Pos, i32, impl Fn(&Pos, &mut dyn FnMut(Pos) -> i32) -> i32> {
    Cache::new(move |&pos, f| {
        let from_geo = |geo| (geo + depth) % 20_183;
        let geo = match pos {
            Pos { x: 0, y: 0 } => 0,
            pos if pos == target => 0,
            Pos { x, y: 0 } => x * 16_807,
            Pos { x: 0, y } => y * 48_271,
            _ => f(pos.moving_to(Dir::North)) * f(pos.moving_to(Dir::West)),
        };
        from_geo(geo)
    })
}

fn part1(depth: i32, target: Pos) -> i32 {
    let mut erosion = erosion(depth, target);
    (0..=target.x)
        .flat_map(|x| (0..=target.y).map(move |y| Pos { x, y }))
        .map(|pos| Type::new(erosion.get(pos)).risk_level())
        .sum()
}

fn part2(depth: i32, target: Pos) -> i32 {
    let mut erosion = Cache::new(|&pos, f| {
        let from_geo = |geo| (geo + depth) % 20_183;
        let geo = match pos {
            Pos { x: 0, y: 0 } => 0,
            pos if pos == target => 0,
            Pos { x, y: 0 } => x * 16_807,
            Pos { x: 0, y } => y * 48_271,
            _ => f(pos.moving_to(Dir::North)) * f(pos.moving_to(Dir::West)),
        };
        from_geo(geo)
    });
    let mut type_of = |pos| Type::new(erosion.get(pos));

    let (_, len) = astar(
        &(Pos::origin(), State::Torch),
        |&(pos, state)| {
            let mut vec: Vec<_> = pos
                .non_neg_neighbors()
                .filter(|&p| state.works_for(type_of(p)))
                .map(|p| ((p, state), 1))
                .collect();
            vec.push(((pos, state.other_state_for(type_of(pos))), 7));
            vec
        },
        |&(pos, _)| pos.manhattan_distance(target),
        |&(pos, state)| pos == target && state == State::Torch,
    )
    .unwrap();

    len
}

fn parser<'a>() -> impl Parser<&'a str, Output = (i32, Pos)> {
    let target = chain((parser::i32(), token(','), parser::i32())).map(|(x, _, y)| Pos { x, y });
    chain((
        string("depth: "),
        parser::i32(),
        string("\ntarget: "),
        target,
    ))
    .map(|(_, depth, _, target)| (depth, target))
}

pub fn solve(input: &str) -> (i32, i32) {
    let (depth, target) = parser().parse_to_end(&input).unwrap();
    (part1(depth, target), part2(depth, target))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2018, 22).await?;
    let (depth, target) = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(depth, target), 4479);
    assert_eq!(part2(depth, target), 1032);
    Ok(())
}
