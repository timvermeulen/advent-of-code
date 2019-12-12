use super::*;

#[derive(Copy, Clone, PartialEq)]
struct Axis {
    p: i32,
    v: i32,
}

impl Axis {
    fn new(p: i32) -> Self {
        Self { p, v: 0 }
    }

    fn apply_gravity(&mut self, other: Self) {
        self.v += (other.p - self.p).signum();
    }

    fn step(&mut self) {
        self.p += self.v;
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
        (x.p.abs() + y.p.abs() + z.p.abs()) * (x.v.abs() + y.v.abs() + z.v.abs())
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
        for i in 0..4 {
            for j in 0..4 {
                if i != j {
                    moons[i].apply_gravity(moons[j]);
                }
            }
        }

        for moon in &mut moons {
            moon.step();
        }
    }
    moons.iter().map(|moon| moon.energy()).sum()
}

fn part2([a, b, c, d]: [Moon; 4]) -> u64 {
    let x_len = cycle_len([a.axes[0], b.axes[0], c.axes[0], d.axes[0]]);
    let y_len = cycle_len([a.axes[1], b.axes[1], c.axes[1], d.axes[1]]);
    let z_len = cycle_len([a.axes[2], b.axes[2], c.axes[2], d.axes[2]]);
    lcm(lcm(x_len, y_len), z_len)
}

fn cycle_len(mut axes: [Axis; 4]) -> u64 {
    let initial = axes;
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
        if axes == initial {
            return step;
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
