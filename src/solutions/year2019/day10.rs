use super::*;
use noisy_float::prelude::*;
use num::integer::gcd;

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

    fn remove(&mut self, point: Point) {
        let index = self.vec.iter().position(|&p| p == point).unwrap();
        self.vec.swap_remove(index);
        self.grid[point.x as usize + self.side_len * point.y as usize] = false;
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

fn part1(asteroids: &Asteroids) -> (Point, usize) {
    let points_between = |p0: Point, p1: Point| {
        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;
        let steps = gcd(dx.abs(), dy.abs());
        (1..steps).map(move |k| Point { x: p0.x + k * dx / steps, y: p0.y + k * dy / steps })
    };

    let is_visible = |p0, p1| p0 != p1 && points_between(p0, p1).all(|p| !asteroids.contains(p));
    asteroids
        .iter()
        .map(|p0| (p0, asteroids.iter().filter(|&p1| is_visible(p0, p1)).count()))
        .max_by_key(|&(_, c)| c)
        .unwrap()
}

fn part2(mut asteroids: Asteroids, laser: Point) -> i32 {
    asteroids.remove(laser);

    let angle_of = |Point { x, y }, angle: N64| -> N64 {
        n64((((x - laser.x) as f64).atan2((laser.y - y) as f64) - angle.raw())
            .rem_euclid(2.0 * std::f64::consts::PI))
    };
    let distance_to = |Point { x, y }| -> i32 { (x - laser.x).abs() + (y - laser.y).abs() };
    let mut angle = n64(0.0);

    (0..200).fold(0, |_, _| {
        let (ast, a) = asteroids
            .iter()
            .map(|ast| (ast, angle_of(ast, angle)))
            .min_by_key(|&(ast, a)| (a, distance_to(ast)))
            .unwrap();
        asteroids.remove(ast);
        angle += a;
        angle += 0.0000001;
        100 * ast.x + ast.y
    })
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 10).await?;
    let asteroids = parse(&input);
    let (laser, count) = part1(&asteroids);
    assert_eq!(count, 278);
    assert_eq!(part2(asteroids, laser), 1417);
    Ok(())
}
