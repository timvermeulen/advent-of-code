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

fn part1(memory: Vec<i64>) -> usize {
    let mut drone = Drone::new(memory);

    let mut pos = Pos { x: 0, y: 0 };
    let mut distance = 0;

    let mut moves = Vec::new();
    let mut states = Vec::new();
    for dir in Dir::all() {
        states.push((dir, 0));
    }

    let mut seen = HashSet::new();
    seen.insert(pos);

    while let Some((dir, dist)) = states.pop() {
        // backtracking
        moves.drain(dist..).rev().for_each(|dir: Dir| {
            drone.move_to(dir.opposite());
            pos.move_to(dir.opposite());
        });

        match drone.move_to(dir) {
            0 => continue,
            1 => {}
            2 => return dist + 1,
            _ => unreachable!(),
        }

        moves.push(dir);
        pos.move_to(dir);
        seen.insert(pos);

        for dir in Dir::all().filter(|&dir| !seen.contains(&pos.moving_to(dir))) {
            states.push((dir, dist + 1));
        }
    }

    unreachable!()
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 15).await?;
    let memory = intcode::parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(memory), 270);
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
            part1(memory)
        });
    }
}
