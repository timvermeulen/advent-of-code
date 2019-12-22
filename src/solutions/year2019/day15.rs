use super::*;
use intcode::prelude::*;

struct Drone {
    comp: Computer,
}

impl Drone {
    fn new(memory: Vec<i64>) -> Self {
        Self { comp: Computer::new(memory) }
    }

    fn move_to(&mut self, dir: Dir) -> i64 {
        let command = match dir {
            Dir::North => 1,
            Dir::South => 2,
            Dir::West => 3,
            Dir::East => 4,
        };
        self.comp.step_with(command).unwrap()
    }
}

fn solve(memory: Vec<i64>) -> (usize, usize) {
    let mut drone = Drone::new(memory);

    let mut pos = Pos::origin();
    let mut distance = 0;

    let mut moves = Vec::new();
    let mut states = Vec::new();
    for dir in Dir::all() {
        states.push((dir, 0));
    }

    let mut seen = HashSet::new();
    seen.insert(pos);

    let mut oxygen = Pos::origin();
    let mut part1 = 0;

    while let Some((dir, dist)) = states.pop() {
        // backtracking
        moves.drain(dist..).rev().for_each(|dir: Dir| {
            drone.move_to(dir.opposite());
            pos.move_to(dir.opposite());
        });

        match drone.move_to(dir) {
            0 => continue,
            1 => {}
            2 => {
                part1 = dist + 1;
                oxygen = pos.moving_to(dir);
            }
            _ => unreachable!(),
        }

        moves.push(dir);
        pos.move_to(dir);
        seen.insert(pos);

        for dir in Dir::all() {
            let n = pos.moving_to(dir);
            if !seen.contains(&n) {
                states.push((dir, dist + 1));
            }
        }
    }

    seen.remove(&oxygen);
    let mut part2 = 0;

    let mut states = vec![(oxygen, 0)];
    while let Some((pos, dist)) = states.pop() {
        part2 = cmp::max(part2, dist);
        for dir in Dir::all() {
            let n = pos.moving_to(dir);
            if seen.remove(&n) {
                states.push((n, dist + 1));
            }
        }
    }

    (part1, part2)
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 15).await?;
    let memory = intcode::parser().parse_to_end(&input).unwrap();
    assert_eq!(solve(memory), (270, 364));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 15)).unwrap();
        b.iter(|| {
            let memory = intcode::parser().parse_to_end(&input).unwrap();
            solve(memory)
        });
    }
}
