use super::*;

#[derive(Copy, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_to(&mut self, dir: Direction) {
        self.x += match dir {
            Direction::Right => 1,
            Direction::Left => -1,
            _ => 0,
        };
        self.y += match dir {
            Direction::Up => 1,
            Direction::Down => -1,
            _ => 0,
        };
    }

    fn manhattan_distance(self) -> u32 {
        self.x.abs() as u32 + self.y.abs() as u32
    }
}

struct Segment {
    dir: Direction,
    len: u32,
}

struct Wire {
    segments: Vec<Segment>,
}

impl Wire {
    fn iter(&self) -> impl Iterator<Item = Position> + '_ {
        self.segments
            .iter()
            .flat_map(|segment| iter::repeat(segment.dir).take(segment.len as usize))
            .scan(Position { x: 0, y: 0 }, |pos, dir| {
                pos.move_to(dir);
                Some(*pos)
            })
    }
}

fn parser<'a>() -> impl Parser<&'a str, Output = [Wire; 2]> {
    let dir = choice((
        token('R').map(|_| Direction::Right),
        token('D').map(|_| Direction::Down),
        token('L').map(|_| Direction::Left),
        token('U').map(|_| Direction::Up),
    ));
    let segment = dir.followed_by(parser::u32()).map(|(dir, len)| Segment { dir, len });
    let wire = segment.collect_sep_by(comma()).map(|segments| Wire { segments });
    chain((wire, newline(), wire)).map(|(a, _, b)| [a, b])
}

fn part1([wire_a, wire_b]: [&Wire; 2]) -> u32 {
    let breadcrumbs: HashSet<_> = wire_a.iter().collect();
    wire_b
        .iter()
        .filter(|pos| breadcrumbs.contains(pos))
        .map(Position::manhattan_distance)
        .min()
        .unwrap()
}

fn part2([wire_a, wire_b]: [&Wire; 2]) -> u32 {
    let breadcrumbs: HashMap<_, u32> = wire_a.iter().zip(1..).collect();
    wire_b
        .iter()
        .zip(1..)
        .filter_map(|(pos, n)| breadcrumbs.get(&pos).map(|m| n + m))
        .min()
        .unwrap()
}

pub fn solve(input: &str) -> (u32, u32) {
    let [wire_a, wire_b] = parser().parse_to_end(&input).unwrap();
    (part1([&wire_a, &wire_b]), part2([&wire_a, &wire_b]))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 3).await?;
    let [wire_a, wire_b] = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1([&wire_a, &wire_b]), 1983);
    assert_eq!(part2([&wire_a, &wire_b]), 107_754);
    Ok(())
}
