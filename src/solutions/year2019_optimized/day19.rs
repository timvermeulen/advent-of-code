use super::*;
use fast_intcode::*;

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x * other.y == other.x * self.y
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (other.x * self.y).partial_cmp(&(self.x * other.y))
    }
}

fn boundary_points(is_pulled: impl Fn(Point) -> bool) -> (Point, Point, Point) {
    if is_pulled(Point { x: 1, y: 1 }) {
        return (Point { x: 1, y: 0 }, Point { x: 1, y: 1 }, Point { x: 0, y: 1 });
    }

    for max in 1.. {
        for i in 1..max {
            let val = max - i;
            let prev = val + 1;
            let bottom = Point { x: val, y: max };
            if is_pulled(bottom) {
                return (Point { x: prev, y: max }, bottom, Point { x: 0, y: 1 });
            }
            let right = Point { x: max, y: val };
            if is_pulled(right) {
                return (Point { x: 1, y: 0 }, right, Point { x: max, y: prev });
            }
        }
    }

    unreachable!()
}

pub fn solve(input: &str) -> (usize, usize) {
    let memory = intcode::parser().parse_to_end(&input).unwrap();
    let is_pulled = |Point { x, y }| {
        let mut computer = Computer::new(memory.clone());
        computer.step_with(x as i64);
        computer.step_with(y as i64).unwrap() == 1
    };

    let (above, inside, under) = boundary_points(is_pulled);

    let mut upper_outside = above;
    let mut upper_inside = inside;
    let mut lower_inside = inside;
    let mut lower_outside = under;

    let mut is_pulled = |x: usize, y: usize| {
        let point = Point { x, y };
        if point <= upper_outside {
            false
        } else if point < upper_inside {
            loop {
                let in_between = Point {
                    x: upper_outside.x + upper_inside.x,
                    y: upper_outside.y + upper_inside.y,
                };
                if is_pulled(in_between) {
                    upper_inside = in_between;
                    if point >= upper_inside {
                        return true;
                    }
                } else {
                    upper_outside = in_between;
                    if point <= upper_outside {
                        return false;
                    }
                }
            }
        } else if point <= lower_inside {
            true
        } else if point < lower_outside {
            loop {
                let in_between = Point {
                    x: lower_outside.x + lower_inside.x,
                    y: lower_outside.y + lower_inside.y,
                };
                if is_pulled(in_between) {
                    lower_inside = in_between;
                    if point <= lower_inside {
                        return true;
                    }
                } else {
                    lower_outside = in_between;
                    if point >= lower_outside {
                        return false;
                    }
                }
            }
        } else {
            false
        }
    };

    let mut part1 = 1;

    for x in 1..50 {
        for y in 1..50 {
            if is_pulled(x, y) {
                part1 += 1;
            }
        }
    }

    let mut min_x = 0;
    let mut max_y = 98;
    loop {
        max_y += 1;
        while !is_pulled(min_x, max_y) {
            min_x += 1;
        }
        let max_x = min_x + 99;
        let min_y = max_y - 99;
        if is_pulled(max_x, min_y) {
            let part2 = min_x * 10_000 + min_y;
            return (part1, part2);
        }
    }
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 19).await?;
    assert_eq!(solve(&input), (234, 9_290_812));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 19)).unwrap();
        b.iter(|| solve(&input));
    }
}
