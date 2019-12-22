use super::*;
use search_algs::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Wall,
    Path,
    Key(u8),
    Door(u8),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Mask(u32);

impl Mask {
    fn new() -> Self {
        Self(0)
    }

    fn contains(self, n: u8) -> bool {
        self.0 & (1 << n) != 0
    }

    fn insert(&mut self, n: u8) {
        self.0 |= 1 << n;
    }

    fn remove(&mut self, n: u8) {
        self.0 &= !(1 << n);
    }

    fn len(self) -> u32 {
        self.0.count_ones()
    }

    fn is_empty(self) -> bool {
        self.0 == 0
    }

    fn iter(mut self) -> impl Iterator<Item = u8> {
        iter::from_fn(move || {
            if self.is_empty() {
                None
            } else {
                let n = self.0.trailing_zeros() as u8;
                self.remove(n);
                Some(n)
            }
        })
    }

    fn add(&mut self, other: Mask) {
        self.0 |= other.0
    }
}

impl Debug for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for key in self.iter() {
            write!(f, "{}", (b'a' + key) as char)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
struct Data {
    accessible_keys: Mask,
    all_keys: Mask,
    blocked_by_count: [u32; 26],
    blocked: [Mask; 26],
    distance: [[i32; 26]; 26],
    distance_to_entrance: [(i32, Pos); 26],
    quadrants: [Mask; 4],
}

impl Data {
    fn distance(&self, location: Location, key: u8) -> i32 {
        match location {
            Location::Entrance => self.distance_to_entrance[key as usize].0,
            Location::Key(k) => self.distance[key as usize][k as usize],
        }
    }

    fn quadrant_of(&self, key: u8) -> usize {
        self.quadrants.iter().position(|mask| mask.contains(key)).unwrap()
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Location {
    Entrance,
    Key(u8),
}

fn data(input: &str) -> Data {
    let mut entrance = Pos::origin();
    let mut grid = Vec::new();
    let width = input.bytes().position(|b| b == b'\n').unwrap();

    let index_of = |Pos { x, y }| x as usize + y as usize * width;

    for (row, y) in input.lines().zip(0..) {
        for (c, x) in row.bytes().zip(0..) {
            let pos = Pos { x, y };
            let tile = match c {
                b'.' => Tile::Path,
                b'#' => Tile::Wall,
                b'@' => {
                    entrance = pos;
                    Tile::Path
                }
                b'a'..=b'z' => Tile::Key(c - b'a'),
                b'A'..=b'Z' => Tile::Door(c - b'A'),
                _ => unreachable!(),
            };
            grid.push(tile);
        }
    }

    for pos in entrance.neighbors() {
        grid[index_of(pos)] = Tile::Wall;
    }

    let mut distance = [[0; 26]; 26];
    let mut distance_to_entrance = [(0, Pos::origin()); 26];
    let mut blocked = [Mask::new(); 26];
    let mut blocked_by_count = [0; 26];
    let mut all_keys = Mask::new();
    let mut quadrants = [Mask::new(); 4];

    for (i, &(dx, dy)) in [(-1, -1), (-1, 1), (1, -1), (1, 1)].iter().enumerate() {
        let entrance = Pos { x: entrance.x + dx, y: entrance.y + dy };
        let mut stack = vec![(entrance, 0, Mask::new())];
        let mut seen = HashSet::new();
        let mut branches = Vec::<(u8, i32, i32)>::new(); // (key, distance to entrance, distance to branch)
        let mut prev_dist = 0;
        let mut quadrant = Mask::new();

        while let Some((pos, dist, mut doors)) = stack.pop() {
            match grid[index_of(pos)] {
                Tile::Wall => continue,
                Tile::Path => {}
                Tile::Key(key) => {
                    quadrant.insert(key);
                    blocked_by_count[key as usize] = doors.len();
                    for door in doors.iter() {
                        blocked[door as usize].insert(key);
                    }
                    distance_to_entrance[key as usize] = (dist, entrance);
                    for &(other, other_dist, branch) in &branches {
                        let between = dist + other_dist - 2 * branch;
                        distance[key as usize][other as usize] = between;
                        distance[other as usize][key as usize] = between;
                    }
                    branches.push((key, dist, dist));
                }
                Tile::Door(door) => {
                    doors.insert(door);
                }
            }

            let branch = dist - 1;
            if branch != prev_dist {
                // bracktracking
                for (_, _, key_branch) in &mut branches {
                    *key_branch = cmp::min(*key_branch, branch);
                }
            }
            prev_dist = dist;

            for neighbor in pos.non_neg_neighbors() {
                if seen.contains(&neighbor) {
                    continue;
                }
                seen.insert(neighbor);
                stack.push((neighbor, dist + 1, doors));
            }
        }

        all_keys.add(quadrant);
        quadrants[i] = quadrant;
    }

    for i in 0..26 {
        for j in i + 1..26 {
            let (dist0, entr0) = distance_to_entrance[i];
            let (dist1, entr1) = distance_to_entrance[j];
            if entr0 != entr1 {
                let dist = dist0 + dist1 + entr0.manhattan_distance(entr1);
                distance[i][j] = dist;
                distance[j][i] = dist;
            }
        }
    }

    let pairwise_distance = |loc: Location, key: u8| match loc {
        Location::Entrance => distance_to_entrance[key as usize].0,
        Location::Key(k) => distance[key as usize][k as usize],
    };

    let mut accessible_keys = Mask::new();
    for i in 0..26 {
        if all_keys.contains(i) && blocked_by_count[i as usize] == 0 {
            accessible_keys.insert(i);
        }
    }

    Data {
        accessible_keys,
        all_keys,
        blocked_by_count,
        blocked,
        distance,
        distance_to_entrance,
        quadrants,
    }
}

fn part1(data: &Data) -> i32 {
    #[derive(Eq, PartialEq, Copy, Clone, Hash)]
    struct Node {
        location: Location,
        accessible_keys: Mask,
        not_collected: Mask,
        blocked_by_count: [u32; 26],
    }

    let (_, cost) = dijkstra(
        &Node {
            location: Location::Entrance,
            accessible_keys: data.accessible_keys,
            not_collected: data.all_keys,
            blocked_by_count: data.blocked_by_count,
        },
        |&node| {
            node.accessible_keys.iter().map(move |key| {
                let mut node = node;
                let cost = data.distance(node.location, key);
                node.location = Location::Key(key);
                node.accessible_keys.remove(key);
                node.not_collected.remove(key);
                for other in data.blocked[key as usize].iter() {
                    node.blocked_by_count[other as usize] -= 1;
                    if node.blocked_by_count[other as usize] == 0 {
                        node.accessible_keys.insert(other);
                    }
                }
                (node, cost)
            })
        },
        |&node| node.not_collected.is_empty(),
    )
    .unwrap();
    cost + 2
}

fn part2(data: &Data) -> i32 {
    #[derive(Eq, PartialEq, Copy, Clone, Hash)]
    struct Node {
        locations: [Location; 4],
        accessible_keys: Mask,
        not_collected: Mask,
        blocked_by_count: [u32; 26],
    }

    let (_, cost) = dijkstra(
        &Node {
            locations: [Location::Entrance; 4],
            accessible_keys: data.accessible_keys,
            not_collected: data.all_keys,
            blocked_by_count: data.blocked_by_count,
        },
        |&node| {
            node.accessible_keys.iter().map(move |key| {
                let quadrant = data.quadrant_of(key);
                let mut node = node;
                let cost = data.distance(node.locations[quadrant], key);
                node.locations[quadrant] = Location::Key(key);
                node.accessible_keys.remove(key);
                node.not_collected.remove(key);
                for other in data.blocked[key as usize].iter() {
                    node.blocked_by_count[other as usize] -= 1;
                    if node.blocked_by_count[other as usize] == 0 {
                        node.accessible_keys.insert(other);
                    }
                }
                (node, cost)
            })
        },
        |&node| node.not_collected.is_empty(),
    )
    .unwrap();
    cost
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 18).await?;
    let data = data(&input);
    assert_eq!(part1(&data), 5858);
    assert_eq!(part2(&data), 2144);
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_data(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 18)).unwrap();
        b.iter(|| data(&input));
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 18)).unwrap();
        b.iter(|| part1(&data(&input)));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 18)).unwrap();
        b.iter(|| part2(&data(&input)));
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 18)).unwrap();
        b.iter(|| {
            let data = data(&input);
            (part1(&data), part2(&data))
        });
    }
}
