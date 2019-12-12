use super::*;
use num::integer::lcm;

#[derive(Copy, Clone, PartialEq)]
struct Axis {
    pos: i32,
    vel: i32,
}

impl Axis {
    fn new(pos: i32) -> Self {
        Self { pos, vel: 0 }
    }

    fn apply_gravity(&mut self, other: Self) {
        self.vel += (other.pos - self.pos).signum();
    }

    fn step(&mut self) {
        self.pos += self.vel;
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Moon {
    axes: [Axis; 3],
}

impl Moon {
    fn apply_gravity(&mut self, other: Moon) {
        for (a, &b) in self.axes.iter_mut().zip(&other.axes) {
            a.apply_gravity(b);
        }
    }

    fn step(&mut self) {
        for a in &mut self.axes {
            a.step();
        }
    }

    fn energy(self) -> i32 {
        let Moon { axes: [x, y, z] } = self;
        (x.pos.abs() + y.pos.abs() + z.pos.abs()) * (x.vel.abs() + y.vel.abs() + z.vel.abs())
    }
}

fn parser<'a>() -> impl Parser<&'a str, Output = [Moon; 4]> {
    let moon = parser::i32()
        .sep_by(rubbish(), |iter| {
            let axes = [Axis::new(iter.next()?), Axis::new(iter.next()?), Axis::new(iter.next()?)];
            Some(Moon { axes })
        })
        .between(rubbish(), rubbish());
    moon.sep_by(newline(), |iter| Some([iter.next()?, iter.next()?, iter.next()?, iter.next()?]))
}

fn part1(mut moons: [Moon; 4]) -> i32 {
    for _ in 0..1000 {
        moons[0].apply_gravity(moons[1]);
        moons[0].apply_gravity(moons[2]);
        moons[0].apply_gravity(moons[3]);
        moons[1].apply_gravity(moons[2]);
        moons[1].apply_gravity(moons[3]);
        moons[1].apply_gravity(moons[0]);
        moons[2].apply_gravity(moons[3]);
        moons[2].apply_gravity(moons[0]);
        moons[2].apply_gravity(moons[1]);
        moons[3].apply_gravity(moons[0]);
        moons[3].apply_gravity(moons[1]);
        moons[3].apply_gravity(moons[2]);

        for moon in &mut moons {
            moon.step();
        }
    }
    moons.iter().map(|moon| moon.energy()).sum()
}

fn part2([a, b, c, d]: [Moon; 4]) -> u64 {
    [0, 1, 2]
        .par_iter()
        .map(|&i| cycle_len([a.axes[i], b.axes[i], c.axes[i], d.axes[i]]))
        .reduce(|| 1, |a, b| lcm(a, b))
}

fn cycle_len(mut axes: [Axis; 4]) -> u64 {
    for step in 1.. {
        for i in 0..4 {
            for j in 0..4 {
                if i != j {
                    axes[i].apply_gravity(axes[j]);
                }
            }
        }
        for axis in &mut axes {
            axis.step();
        }
        if axes[0].vel == 0 && axes[1].vel == 0 && axes[2].vel == 0 {
            return step * 2;
        }
    }
    unreachable!()
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 12).await?;
    let moons = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(moons), 7758);
    assert_eq!(part2(moons), 354_540_398_381_256);
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 12)).unwrap();
        b.iter(|| {
            let moons = parser().parse_to_end(&input).unwrap();
            part1(moons)
        });
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 12)).unwrap();
        b.iter(|| {
            let moons = parser().parse_to_end(&input).unwrap();
            part2(moons)
        });
    }

    #[bench]
    fn bench_both(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 12)).unwrap();
        b.iter(|| {
            let moons = parser().parse_to_end(&input).unwrap();
            (part1(moons), part2(moons))
        });
    }
}
