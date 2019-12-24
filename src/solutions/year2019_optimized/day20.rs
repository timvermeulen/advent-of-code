use super::*;
use mask::*;
use search_algs::*;

// IMPORTANT:
// don't bother reading this code, it's not supposed to make any sense

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct PortalID([u8; 2]);

impl Display for PortalID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [a, b] = self.0;
        write!(f, "{}{}", a as char, b as char)
    }
}

struct PortalInfo {
    teleports_to: usize,
    cost_of_teleportation: u32,
    depth_change: i32,
    accessible_portals: Mask<u64>,
}

struct Data {
    start: usize,
    end: usize,
    distances: Vec<u32>,
    portals: Vec<PortalInfo>,
    min_dist: u32,
}

impl Data {
    fn portals_accessible_from(&self, portal: usize) -> impl Iterator<Item = u32> + '_ {
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

    let is_in_corner = |Pos { x, y }| {
        let w = width as i32;
        let h = height as i32;
        let t = thickness as i32 + 2;
        x < t && y < t || x < t && y >= h - t || x >= w - t && y >= h - t || x >= w - t && y < t
    };

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

    let mut portal_info: Vec<_> = (0..num_portals)
        .map(|_| PortalInfo {
            teleports_to: 0,
            cost_of_teleportation: 0,
            depth_change: 0,
            accessible_portals: Mask::empty(),
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
    let mut remaining_portals = Mask::<u64>((1 << num_portals) - 1);

    while let Some(i) = remaining_portals.iter().next() {
        remaining_portals.remove(i);
        let i = i as usize;
        let (_id, pos, dir) = portals[i];

        stack.clear();
        stack.push((pos.moving_to(dir), 0, dir));

        let mut prev_steps = 0;
        let mut other_portal = 0;
        let mut only_steps = 0;

        // Option<(portal, distance to entrance, distance to branch)>
        let mut second = None;
        let mut third = None;
        let mut fourth_found = false;

        while let Some((pos, steps, dir)) = stack.pop() {
            let dist = 2 * steps + 1;
            if steps != prev_steps + 1 {
                // bracktracking
                if let Some((_, _, ref mut branch_dist)) = second {
                    *branch_dist = cmp::min(*branch_dist, dist - 2);

                    if let Some((_, _, ref mut branch_dist)) = third {
                        *branch_dist = cmp::min(*branch_dist, dist - 2);
                    }
                }
            }
            prev_steps = steps;

            for &dir in &[dir, dir.left(), dir.right()] {
                let n = pos.moving_to(dir);
                if is_path(n) && !is_in_corner(n) {
                    let mut is_portal = false;

                    for j in remaining_portals.iter() {
                        let j = j as usize;
                        if n == portals[j].1 {
                            is_portal = true;
                            remaining_portals.remove(j as u32);
                            set_distance(i, j, dist + 1);
                            other_portal = j;
                            only_steps = steps;

                            if let Some((k, a, b)) = second {
                                set_distance(j, k, dist + 2 + a - 2 * b);

                                if let Some((k, a, b)) = third {
                                    set_distance(j, k, dist + 2 + a - 2 * b);
                                    fourth_found = true;
                                } else {
                                    third = Some((j, dist, dist));
                                }
                            } else {
                                second = Some((j, dist, dist));
                            }
                            break;
                        }
                    }

                    if !is_portal {
                        stack.push((n.moving_to(dir), steps + 1, dir));
                    }
                }
            }

            if fourth_found {
                break;
            }
        }

        if third == None {
            // this room only has two portals
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

            let cost = cost0 + cost1 + 2 * only_steps + 2;

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
                portal_info[i].accessible_portals.insert(j as u32);
                portal_info[j].accessible_portals.insert(i as u32);
            }
        }
    }

    let mut min_dist = u32::max_value();
    for i in 0..num_portals {
        for j in portal_info[i].accessible_portals.iter() {
            let j = j as usize;
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
                let j = j as usize;
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
                let j = j as usize;
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
        |&(i, depth)| i == data.end && depth == 0,
    )
    .unwrap();
    len
}

pub fn solve(input: &str) -> (u32, u32) {
    let data = data(&input);
    (part1(&data), part2(&data))
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
