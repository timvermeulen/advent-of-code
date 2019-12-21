use super::*;
use search_algs::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct PortalID([u8; 2]);

impl Display for PortalID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [a, b] = self.0;
        write!(f, "{}{}", a as char, b as char)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Mask(u64);

impl Mask {
    fn new() -> Self {
        Self(0)
    }

    fn contains(self, n: usize) -> bool {
        self.0 & (1 << n) != 0
    }

    fn insert(&mut self, n: usize) {
        self.0 |= 1 << n;
    }

    fn remove(&mut self, n: usize) {
        self.0 &= !(1 << n);
    }

    fn len(self) -> u32 {
        self.0.count_ones()
    }

    fn is_empty(self) -> bool {
        self.0 == 0
    }

    fn first(self) -> Option<usize> {
        if self.is_empty() {
            None
        } else {
            Some(self.0.trailing_zeros() as usize)
        }
    }

    fn iter(mut self) -> impl Iterator<Item = usize> {
        iter::from_fn(move || {
            let n = self.first()?;
            self.remove(n);
            Some(n)
        })
    }

    fn add(&mut self, other: Mask) {
        self.0 |= other.0
    }
}

struct PortalInfo {
    id: PortalID,
    teleports_to: usize,
    cost_of_teleportation: u32,
    depth_change: i32,
    accessible_portals: Mask,
}

struct Data {
    start: usize,
    end: usize,
    distances: Vec<u32>,
    portals: Vec<PortalInfo>,
    min_dist: u32,
}

impl Data {
    fn portals_accessible_from(&self, portal: usize) -> impl Iterator<Item = usize> + '_ {
        self.portals[portal].accessible_portals.iter()
    }

    fn distance_between(&self, i: usize, j: usize) -> u32 {
        self.distances[i + j * self.portals.len()]
    }

    fn sibling_portal_of(&self, portal: usize) -> usize {
        self.portals[portal].teleports_to
    }

    fn cost_of(&self, portal: usize) -> u32 {
        self.portals[portal].cost_of_teleportation
    }

    fn depth_change_of(&self, portal: usize) -> i32 {
        self.portals[portal].depth_change
    }
}

fn data(input: &str) -> Data {
    let mut start = 0;
    let mut end = 0;
    let mut portals = Vec::new(); // (id, pos, dir)

    let width = input.bytes().position(|b| b == b'\n').unwrap();
    let height = (input.len() + 1) / (width + 1);

    let byte_at = |Pos { x, y }| input.as_bytes()[y as usize * (width + 1) + x as usize];
    let is_path = |pos| byte_at(pos) == b'.';
    let is_outside =
        |Pos { x, y }| x == 2 || x == width as i32 - 3 || y == 2 || y == height as i32 - 3;

    let mut add_portal = |x: usize, y: usize, dir: Dir| {
        let pos = Pos { x: x as i32, y: y as i32 };

        if !is_path(pos) {
            return;
        }

        let n = pos.moving_to(dir);
        let b0 = byte_at(n);
        let b1 = byte_at(n.moving_to(dir));

        let id = match dir {
            Dir::North | Dir::West => PortalID([b1, b0]),
            Dir::South | Dir::East => PortalID([b0, b1]),
        };

        let index = portals.len();

        match id {
            PortalID([b'A', b'A']) => start = index,
            PortalID([b'Z', b'Z']) => end = index,
            _ => {}
        }

        portals.push((id, pos, dir.opposite()));
    };

    let thickness = (2..).find(|&i| byte_at(Pos { x: i, y: i }) == b' ').unwrap() as usize - 2;

    for x in 2..width - 2 {
        add_portal(x, 2, Dir::North);
        add_portal(x, height as usize - 3, Dir::South);
    }

    for x in thickness + 2..width - thickness - 2 {
        add_portal(x, thickness + 1, Dir::South);
        add_portal(x, height as usize - thickness - 2, Dir::North);
    }

    for y in 2..height - 2 {
        add_portal(2, y, Dir::West);
        add_portal(width as usize - 3, y, Dir::East);
    }

    for y in thickness + 2..height - thickness - 2 {
        add_portal(thickness + 1, y, Dir::East);
        add_portal(width as usize - thickness - 2, y, Dir::West);
    }

    let num_portals = portals.len();

    let mut portal_info: Vec<_> = portals
        .iter()
        .map(|&(id, _, _)| PortalInfo {
            id,
            teleports_to: 0,
            cost_of_teleportation: 0,
            depth_change: 0,
            accessible_portals: Mask::new(),
        })
        .collect();

    portal_info[start].teleports_to = start;
    portal_info[end].teleports_to = end;

    for i in 0..num_portals {
        let (id, pos, _dir) = portals[i];
        let delta = if is_outside(pos) { -1 } else { 1 };
        for j in 0..num_portals {
            if i != j && id == portals[j].0 {
                let info = &mut portal_info[i];
                info.teleports_to = j;
                info.cost_of_teleportation = 1;
                info.depth_change = delta;
            }
        }
    }

    let mut distances = vec![0; num_portals * num_portals];
    let mut set_distance = |i: usize, j: usize, dist| {
        distances[i + j * num_portals] = dist;
        distances[j + i * num_portals] = dist;
    };

    let mut stack = Vec::new();
    let mut other_points = Vec::new(); // (key, distance to entrance, distance to branch)

    let mut remaining_portals = Mask((1 << num_portals) - 1);

    while let Some(i) = remaining_portals.first() {
        remaining_portals.remove(i);
        let (_id, pos, dir) = portals[i];

        stack.clear();
        stack.push((pos, 0, dir));
        other_points.clear();

        let mut prev_dist = 0;
        let mut other_portal = 0;
        let mut only_dist = 0;

        while let Some((pos, dist, dir)) = stack.pop() {
            if dist != prev_dist + 1 {
                // bracktracking
                for (_, _, branch_dist) in &mut other_points {
                    *branch_dist = cmp::min(*branch_dist, dist - 1);
                }
            }
            prev_dist = dist;

            for j in remaining_portals.iter() {
                if pos == portals[j].1 {
                    remaining_portals.remove(j);
                    set_distance(i, j, dist);
                    other_portal = j;
                    only_dist = dist;

                    for &(k, a, b) in &other_points {
                        set_distance(j, k, dist + a - 2 * b);
                    }

                    other_points.push((j, dist, dist));
                }
            }

            for &dir in &[dir, dir.left(), dir.right()] {
                let n = pos.moving_to(dir);
                if is_path(n) {
                    stack.push((n, dist + 1, dir));
                }
            }
        }

        if other_points.len() == 1 {
            // this room has only two portals, `i` and `other_portal`
            let PortalInfo {
                teleports_to: i0,
                cost_of_teleportation: cost0,
                depth_change: delta0,
                ..
            } = portal_info[i];

            let PortalInfo {
                teleports_to: i1,
                cost_of_teleportation: cost1,
                depth_change: delta1,
                ..
            } = portal_info[other_portal];

            let cost = cost0 + cost1 + only_dist;

            let p0 = &mut portal_info[i0];
            p0.teleports_to = i1;
            p0.cost_of_teleportation = cost;
            p0.depth_change += delta1;

            let p1 = &mut portal_info[i1];
            p1.teleports_to = i0;
            p1.cost_of_teleportation = cost;
            p1.depth_change += delta0;
        }
    }

    for i in 0..num_portals {
        for j in i + 1..num_portals {
            let dist = distances[i * num_portals + j];
            if dist > 0 {
                portal_info[i].accessible_portals.insert(j);
                portal_info[j].accessible_portals.insert(i);
            }
        }
    }

    let mut min_dist = u32::max_value();
    for i in 0..num_portals {
        for j in portal_info[i].accessible_portals.iter() {
            if portal_info[i].depth_change == 1 && portal_info[j].depth_change == -1 {
                min_dist = cmp::min(min_dist, distances[i * num_portals + j]);
            }
        }
    }

    Data { start, end, distances, portals: portal_info, min_dist }
}

fn part1(data: &Data) -> u32 {
    let (_, len) = dijkstra(
        &data.start,
        |&i| {
            data.portals_accessible_from(i).filter_map(move |j| {
                let dist = data.distance_between(i, j);
                let sibling = data.sibling_portal_of(j);
                let cost = data.cost_of(j);
                if dist > 0 {
                    Some((sibling, dist + cost))
                } else {
                    None
                }
            })
        },
        |&i| i == data.end,
    )
    .unwrap();
    len
}

fn part2(data: &Data) -> u32 {
    let (_, len) = astar(
        &(data.start, 0),
        |&(i, depth)| {
            data.portals_accessible_from(i).filter_map(move |j| {
                let dist = data.distance_between(i, j);
                let delta = data.depth_change_of(j);
                let new_depth = depth + delta;
                let is_blocked = delta == 0 && depth > 0;
                if !is_blocked && dist > 0 && new_depth >= 0 {
                    Some(((data.sibling_portal_of(j), new_depth), dist + data.cost_of(j)))
                } else {
                    None
                }
            })
        },
        |&(_, depth)| depth as u32 * (data.min_dist + 1),
        |&(i, depth)| {
            i == data.end && depth == 0
        },
    )
    .unwrap();
    len
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 20).await?;
    let data = data(&input);
    assert_eq!(part1(&data), 410);
    assert_eq!(part2(&data), 5084);
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 20)).unwrap();
        b.iter(|| {
            let data = data(&input);
            (part1(&data), part2(&data))
        });
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 20)).unwrap();
        let data = data(&input);
        b.iter(|| part2(&data));
    }
}
