use super::*;
use mask::Mask;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Wall,
    Path,
    Key(u32),
    Door(u32),
}

#[derive(Copy, Clone)]
struct Data {
    all_keys: Mask<u32>,
    blocked_by: [Mask<u32>; 26],
    distance: [[i32; 26]; 26],
    distance_to_entrance: [(i32, Pos); 26],
    quadrants: [Mask<u32>; 4],
}

impl Data {
    fn distance_between(&self, location: Location, key: u32) -> i32 {
        match location {
            Location::Entrance => self.distance_to_entrance[key as usize].0,
            Location::Key(k) => self.distance[key as usize][k as usize],
        }
    }

    fn quadrant_of(&self, key: u32) -> usize {
        self.quadrants
            .iter()
            .position(|mask| mask.contains(key))
            .unwrap()
    }

    fn doors_blocking_key(&self, key: u32) -> Mask<u32> {
        self.blocked_by[key as usize]
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Location {
    Entrance,
    Key(u32),
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
                b'a'..=b'z' => Tile::Key((c - b'a') as u32),
                b'A'..=b'Z' => Tile::Door((c - b'A') as u32),
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
    let mut blocked_by = [Mask::empty(); 26];
    let mut all_keys = Mask::empty();
    let mut quadrants = [Mask::empty(); 4];

    for (i, &(dx, dy)) in [(-1, -1), (-1, 1), (1, -1), (1, 1)].iter().enumerate() {
        let entrance = Pos {
            x: entrance.x + dx,
            y: entrance.y + dy,
        };
        let mut stack = vec![(entrance, 0, Mask::empty())];
        let mut seen = HashSet::new();
        let mut branches = Vec::<(u32, i32, i32)>::new(); // (key, distance to entrance, distance to branch)
        let mut prev_dist = 0;
        let mut quadrant = Mask::empty();

        while let Some((pos, dist, mut doors)) = stack.pop() {
            match grid[index_of(pos)] {
                Tile::Wall => continue,
                Tile::Path => {}
                Tile::Key(key) => {
                    quadrant.insert(key);
                    blocked_by[key as usize] = doors;
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

    Data {
        all_keys,
        blocked_by,
        distance,
        distance_to_entrance,
        quadrants,
    }
}

fn part1(data: &Data) -> i32 {
    #[derive(Eq, PartialEq, Copy, Clone, Hash)]
    struct Node {
        location: Location,
        not_collected: Mask<u32>,
    }

    let mut map = HashMap::new();
    let mut new = HashMap::new();
    map.insert(
        Node {
            location: Location::Entrance,
            not_collected: data.all_keys,
        },
        2,
    );

    for _ in 0..26 {
        for (node, cost) in map.drain() {
            for key in node.not_collected.iter() {
                if data.doors_blocking_key(key).intersects(node.not_collected) {
                    continue;
                }

                let mut node = node;
                let cost = cost + data.distance_between(node.location, key);
                node.location = Location::Key(key);
                node.not_collected.remove(key);
                new.entry(node)
                    .and_modify(|c| *c = cmp::min(*c, cost))
                    .or_insert(cost);
            }
        }
        mem::swap(&mut map, &mut new);
    }

    map.values().copied().min().unwrap()
}

fn part2(data: &Data) -> i32 {
    #[derive(Eq, PartialEq, Copy, Clone, Hash)]
    struct Node {
        locations: [Location; 4],
        not_collected: Mask<u32>,
    }

    let mut map = HashMap::new();
    let mut new = HashMap::new();
    map.insert(
        Node {
            locations: [Location::Entrance; 4],
            not_collected: data.all_keys,
        },
        0,
    );

    for _ in 0..26 {
        for (node, cost) in map.drain() {
            for key in node.not_collected.iter() {
                if data.doors_blocking_key(key).intersects(node.not_collected) {
                    continue;
                }

                let quadrant = data.quadrant_of(key);
                let mut node = node;
                let cost = cost + data.distance_between(node.locations[quadrant], key);
                node.locations[quadrant] = Location::Key(key);
                node.not_collected.remove(key);
                new.entry(node)
                    .and_modify(|c| *c = cmp::min(*c, cost))
                    .or_insert(cost);
            }
        }
        mem::swap(&mut map, &mut new);
    }

    map.values().copied().min().unwrap()
}

pub fn solve(input: &str) -> (i32, i32) {
    let data = data(&input);
    (part1(&data), part2(&data))
}

pub fn solve_part1(input: &str) -> i32 {
    let data = data(&input);
    part1(&data)
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
