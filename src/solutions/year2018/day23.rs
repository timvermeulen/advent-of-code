use super::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn distance_to(&self, position: Point) -> i32 {
        (self.x - position.x).abs() + (self.y - position.y).abs() + (self.z - position.z).abs()
    }

    fn distance_to_origin(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance_to_origin().cmp(&(other.distance_to_origin()))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A cube with a side length of a power of 2.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Cube {
    origin: Point,
    side_length: i32,
}

impl Cube {
    fn new(origin: Point, exponent: u32) -> Cube {
        let side_length = 1 << exponent;

        Cube { origin, side_length }
    }

    /// Splits the cube into smaller cubes each with half the side
    /// length of the original cube.
    fn split(&self) -> [Self; 8] {
        let side_length = self.side_length / 2;
        let cube = |x: bool, y: bool, z: bool| -> Self {
            let extra = |flag: bool| if flag { side_length } else { 0 };
            Self {
                origin: Point {
                    x: self.origin.x + extra(x),
                    y: self.origin.y + extra(y),
                    z: self.origin.z + extra(z),
                },
                side_length,
            }
        };
        [
            cube(false, false, false),
            cube(false, false, true),
            cube(false, true, false),
            cube(false, true, true),
            cube(true, false, false),
            cube(true, false, true),
            cube(true, true, false),
            cube(true, true, true),
        ]
    }

    /// Returns the origin if that's the only point inside the cube,
    /// and `None` otherwise.
    fn only_point(&self) -> Option<Point> {
        if self.side_length == 1 {
            Some(self.origin)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct NanoBot {
    position: Point,
    radius: i32,
}

impl NanoBot {
    /// Returns `true` if the given point is in range of the bot.
    fn reaches_point(&self, point: Point) -> bool {
        self.position.distance_to(point) <= self.radius
    }

    /// Returns `true` if any point of the cube is in range of the bot.
    fn reaches_cube(&self, cube: Cube) -> bool {
        let distance = |bot: i32, cube_origin: i32| {
            if bot < cube_origin {
                cube_origin - bot
            } else if bot >= cube_origin + cube.side_length {
                bot - cube_origin - cube.side_length + 1
            } else {
                0
            }
        };

        distance(self.position.x, cube.origin.x)
            + distance(self.position.y, cube.origin.y)
            + distance(self.position.z, cube.origin.z)
            <= self.radius
    }
}

/// A cube that can be compared based on how many bots are in range of it.
#[derive(Debug)]
struct OrdCube {
    cube: Cube,
    bots_in_range: usize,
}

impl PartialEq for OrdCube {
    fn eq(&self, other: &Self) -> bool {
        self.cube == other.cube
    }
}

impl Eq for OrdCube {}

impl Ord for OrdCube {
    fn cmp(&self, other: &OrdCube) -> Ordering {
        self.bots_in_range
            .cmp(&other.bots_in_range)
            // if both cubes have the same number of bots in range,
            // the one closer to the origin is considered "larger"
            .then(other.cube.origin.cmp(&self.cube.origin))
    }
}

impl PartialOrd for OrdCube {
    fn partial_cmp(&self, other: &OrdCube) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl OrdCube {
    fn new(cube: Cube, bots: &[NanoBot]) -> OrdCube {
        let bots_in_range = bots.iter().filter(|b| b.reaches_cube(cube)).count();
        OrdCube { cube, bots_in_range }
    }

    /// Splits the cube into smaller cubes each with half the side
    /// length of the original cube.
    fn split(&self, bots: &[NanoBot]) -> Vec<Self> {
        self.cube.split().iter().map(|&c| OrdCube::new(c, bots)).collect()
    }

    /// Returns the origin if that's the only point inside the cube,
    /// and `None` otherwise.
    fn only_point(&self) -> Option<Point> {
        self.cube.only_point()
    }
}

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<NanoBot>> {
    let position = parser::i32()
        .sep_by(token(','), |iter| {
            Some(Point { x: iter.next()?, y: iter.next()?, z: iter.next()? })
        })
        .between(token('<'), token('>'));
    let bot = chain((string("pos="), position, string(", r="), parser::i32()))
        .map(|(_, position, _, radius)| NanoBot { position, radius });
    bot.collect_sep_by(token('\n'))
}

fn part1(bots: &[NanoBot]) -> u32 {
    let bot = bots.iter().max_by_key(|bot| bot.radius).unwrap();
    bots.iter().filter(|b| bot.reaches_point(b.position)).count() as u32
}

fn part2(bots: &[NanoBot]) -> i32 {
    let xs = || bots.iter().map(|b| b.position.x);
    let ys = || bots.iter().map(|b| b.position.y);
    let zs = || bots.iter().map(|b| b.position.z);

    let min_x = xs().min().unwrap();
    let max_x = xs().max().unwrap();
    let min_y = ys().min().unwrap();
    let max_y = ys().max().unwrap();
    let min_z = zs().min().unwrap();
    let max_z = zs().max().unwrap();

    let size = cmp::max(cmp::max(max_x - min_x, max_y - min_y), max_z - min_z);
    let exponent = 8 * mem::size_of::<i32>() as u32 - size.leading_zeros();
    let origin = Point { x: min_x, y: min_y, z: min_z };

    let cube = OrdCube::new(Cube::new(origin, exponent), &bots);
    let mut heap = BinaryHeap::new();
    heap.push(cube);

    while let Some(cube) = heap.pop() {
        match cube.only_point() {
            None => cube.split(bots).into_iter().for_each(|c| heap.push(c)),
            Some(point) => return point.distance_to_origin(),
        }
    }

    unreachable!()
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2018, 23).await?;
    let bots = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&bots), 761);
    assert_eq!(part2(&bots), 89_915_526);
    Ok(())
}
