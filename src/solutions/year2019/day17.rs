use super::*;
use intcode::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Turn {
    Left,
    Right,
}

impl Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Left => write!(f, "L"),
            Self::Right => write!(f, "R"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Segment {
    turn: Turn,
    len: u32,
}

impl Segment {
    fn input_len(self) -> usize {
        3 + (self.len >= 10) as usize
    }
}

fn find_routines_<'a>(
    segments: &'a [Segment],
    routines: [&'a [Segment]; 3],
    count: usize,
    order: &mut Vec<usize>,
) -> Option<[&'a [Segment]; 3]> {
    if order.len() > 10 {
        return None;
    }
    if segments.is_empty() {
        return Some(routines);
    }
    for i in 3..=segments.len() {
        let (routine, rem) = segments.split_at(i);
        if routine.iter().map(|s| s.input_len()).sum::<usize>() + routine.len() > 21 {
            break;
        }
        if let Some(i) = routines.iter().take(count).position(|&r| r == routine) {
            order.push(i);
            if let Some(x) = find_routines_(rem, routines, count, order) {
                return Some(x);
            }
            order.pop();
        } else if count < 3 {
            let mut routines = routines;
            routines[count] = routine;
            order.push(count);
            if let Some(x) = find_routines_(rem, routines, count + 1, order) {
                return Some(x);
            }
            order.pop();
        }
    }
    None
}

fn find_routines(segments: &[Segment]) -> (Vec<usize>, [&[Segment]; 3]) {
    let mut order = Vec::new();
    let routines = find_routines_(segments, [&[]; 3], 0, &mut order).expect("no answer found");
    (order, routines)
}

pub fn solve(input: &str) -> (i32, i64) {
    let memory = intcode::parser().parse_to_end(&input).unwrap();
    let mut robot = Computer::new(memory);

    let mut grid = Grid::new(false);
    let visual: String = robot.run().map(|o| o as u8 as char).collect();
    let mut start = None;
    for (line, y) in visual.lines().zip(0..) {
        for (b, x) in line.bytes().zip(0..) {
            let pos = Pos { x, y };
            let dir = match b {
                b'.' => continue,
                b'#' => {
                    grid[pos] = true;
                    continue;
                }
                b'^' => Dir::North,
                b'v' => Dir::South,
                b'<' => Dir::West,
                b'>' => Dir::East,
                _ => unreachable!(),
            };
            start = Some((pos, dir));
        }
    }

    let mut part1 = 0;
    for (pos, _) in grid.iter() {
        if pos.non_neg_neighbors().all(|n| grid[n]) {
            part1 += pos.x * pos.y;
        }
    }
    let (mut pos, mut dir) = start.unwrap();

    let mut segments = Vec::new();
    loop {
        let turn = if grid[pos.moving_to(dir.left())] {
            dir.turn_left();
            Turn::Left
        } else if grid[pos.moving_to(dir.right())] {
            dir.turn_right();
            Turn::Right
        } else {
            break;
        };
        let mut len = 0;
        while grid[pos.moving_to(dir)] {
            len += 1;
            pos.move_to(dir);
        }
        segments.push(Segment { turn, len });
    }

    let (order, routines) = find_routines(&segments);
    let mut input = String::new();

    for i in order {
        input.push(['A', 'B', 'C'][i]);
        input.push(',');
    }
    input.pop();
    input.push('\n');

    for &routine in &routines {
        for Segment { turn, len } in routine {
            input.push_str(&format!("{},{},", turn, len));
        }
        input.pop();
        input.push('\n');
    }

    input.push_str("n\n");

    robot.memory[0] = 2;
    let part2 = robot.run_with_iter(input.bytes().map(|b| b as i64)).last().unwrap();

    (part1, part2)
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 17).await?;
    assert_eq!(solve(&input), (2080, 742_673));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 17)).unwrap();
        b.iter(|| solve(&input));
    }
}
