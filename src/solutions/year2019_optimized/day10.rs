use super::*;
use noisy_float::prelude::*;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Asteroids {
    vec: Vec<Point>,
    grid: Vec<bool>,
    side_len: usize,
}

impl Asteroids {
    fn new(side_len: usize) -> Self {
        Self { vec: vec![], grid: vec![false; side_len * side_len], side_len }
    }
    fn iter(&self) -> impl Iterator<Item = Point> + '_ {
        self.vec.iter().copied()
    }

    fn contains(&self, Point { x, y }: Point) -> bool {
        self.grid[x as usize + self.side_len * y as usize]
    }

    fn insert(&mut self, point: Point) {
        self.vec.push(point);
        self.grid[point.x as usize + self.side_len * point.y as usize] = true;
    }
}

struct Grid<T> {
    vec: Vec<T>,
    side_len: usize,
}

impl<T: Clone> Grid<T> {
    fn new(def: T, side_len: usize) -> Self {
        Self { vec: vec![def; (side_len * 2 - 1) * (side_len * 2 - 1)], side_len }
    }
}

impl<T> Grid<T> {
    fn index_of(&self, Point { x, y }: Point) -> usize {
        let offset = self.side_len as i32 - 1;
        let col = x + offset;
        let row = y + offset;
        (row + col * (offset * 2 + 1)) as usize
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;
    fn index(&self, point: Point) -> &Self::Output {
        &self.vec[self.index_of(point)]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        let i = self.index_of(point);
        &mut self.vec[i]
    }
}

fn parse(input: &str) -> Asteroids {
    let mut asteroids = Asteroids::new(input.lines().next().unwrap().len());
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.insert(Point { x: x as i32, y: y as i32 });
            }
        }
    }
    asteroids
}

fn gcd(x: i32, y: i32) -> i32 {
    let mut x = x.abs();
    let mut y = y.abs();
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

fn part1(asteroids: &Asteroids) -> (Point, usize) {
    let size = asteroids.side_len;
    let max = size as i32 - 1;
    let min = -max;

    let mut gcd: Vec<i32> =
        (0..size).flat_map(|y| (0..size).map(move |x| gcd(x as i32, y as i32))).collect();

    let mut normalized = Grid::<Point>::new(Point { x: 0, y: 0 }, asteroids.side_len);
    for x in min..=max {
        for y in min..=max {
            let steps = gcd[x.abs() as usize + y.abs() as usize * size];
            normalized[Point { x, y }] = match steps {
                0 => Point { x: 0, y: 0 },
                _ => Point { x: x / steps, y: y / steps },
            };
        }
    }

    let mut visible = Grid::<usize>::new(0, size);

    asteroids
        .iter()
        .zip(1..)
        .map(|(p0, i)| {
            let mut count = 0;
            for p1 in asteroids.iter() {
                let n = normalized[Point { x: p1.x - p0.x, y: p1.y - p0.y }];
                let entry = &mut visible[n];
                if *entry != i {
                    *entry = i;
                    count += 1;
                }
            }
            (p0, count - 1)
        })
        .max_by_key(|&(_, c)| c)
        .unwrap()
}

fn area(Point { x, y }: Point) -> u32 {
    match (x.signum(), y.signum()) {
        (0, -1) => 0,
        (1, -1) => 1,
        (1, 0) => 2,
        (1, 1) => 3,
        (0, 1) => 4,
        (-1, 1) => 5,
        (-1, 0) => 6,
        (-1, -1) => 7,
        (0, 0) => 8,
        _ => unreachable!(),
    }
}

fn part2(asteroids: &Asteroids, laser: Point) -> i32 {
    let points_between = |p0: Point, p1: Point| {
        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;
        let steps = gcd(dx.abs(), dy.abs());
        (1..steps).map(move |k| Point { x: p0.x + k * dx / steps, y: p0.y + k * dy / steps })
    };

    let is_visible = |p0, p1| p0 != p1 && points_between(p0, p1).all(|p| !asteroids.contains(p));
    let mut visible: Vec<_> = asteroids.iter().filter(|&p| is_visible(laser, p)).collect();
    visible.sort_by(|p0, p1| {
        let x0 = p0.x - laser.x;
        let y0 = p0.y - laser.y;
        let x1 = p1.x - laser.x;
        let y1 = p1.y - laser.y;
        let lhs = (area(Point { x: x0, y: y0 }), x1 * y0);
        let rhs = (area(Point { x: x1, y: y1 }), x0 * y1);
        lhs.cmp(&rhs)
    });
    let asteroid = visible[199];
    100 * asteroid.x + asteroid.y
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 10).await?;
    let asteroids = parse(&input);
    let (laser, count) = part1(&asteroids);
    assert_eq!(count, 278);
    assert_eq!(part2(&asteroids, laser), 1417);
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 10)).unwrap();
        b.iter(|| {
            let asteroids = parse(&input);
            let (_, count) = part1(&asteroids);
            count
        });
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 10)).unwrap();
        let asteroids = parse(&input);
        let (laser, _) = part1(&asteroids);
        b.iter(|| part2(&asteroids, laser));
    }

    #[bench]
    fn bench_both(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 10)).unwrap();
        b.iter(|| {
            let asteroids = parse(&input);
            let (laser, count) = part1(&asteroids);
            (count, part2(&asteroids, laser))
        });
    }
}
