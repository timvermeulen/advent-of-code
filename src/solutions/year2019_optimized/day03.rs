use super::*;

enum HDirection {
    Left,
    Right,
}

enum VDirection {
    Up,
    Down,
}

enum Direction {
    Horizontal(HDirection),
    Vertical(VDirection),
}

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn manhattan_distance(self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

struct Segment {
    dir: Direction,
    len: i32,
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct HSegment {
    min_x: i32,
    max_x: i32,
    start_x: i32,
    y: i32,
    dist: i32,
}

impl PartialOrd for HSegment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HSegment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.max_x.cmp(&other.max_x).reverse()
    }
}

#[derive(Copy, Clone)]
struct VSegment {
    min_y: i32,
    max_y: i32,
    start_y: i32,
    x: i32,
    dist: i32,
}

struct Wire {
    h_segments: Vec<HSegment>,
    v_segments: Vec<VSegment>,
}

// see https://en.wikipedia.org/wiki/Bentley–Ottmann_algorithm
fn intersections(h_segments: &[HSegment], v_segments: &[VSegment]) -> Vec<(i32, i32)> {
    let mut upcoming_h_segments = h_segments.iter().copied().peekable();
    let mut current_h_segments: BinaryHeap<HSegment> = BinaryHeap::new();
    let mut intersections = Vec::new();
    for v_segment in v_segments {
        // remove horizontal segments too far left
        while current_h_segments.peek().filter(|s| s.max_x <= v_segment.x).is_some() {
            current_h_segments.pop();
        }

        // add new horizontal segments that may overlap
        while upcoming_h_segments.peek().filter(|s| s.min_x < v_segment.x).is_some() {
            let s = upcoming_h_segments.next().unwrap();

            // the horizontal segment may already be to the left of the vertical segment
            if s.max_x > v_segment.x {
                current_h_segments.push(s);
            }
        }

        intersections.extend(
            current_h_segments
                .iter()
                .filter(|h_segment| v_segment.min_y < h_segment.y && h_segment.y < v_segment.max_y)
                .map(|h_segment| {
                    let intersection = Position { x: v_segment.x, y: h_segment.y };
                    let h_dist = h_segment.dist + (h_segment.start_x - intersection.x).abs();
                    let v_dist = v_segment.dist + (v_segment.start_y - intersection.y).abs();
                    (intersection.manhattan_distance(), h_dist + v_dist)
                }),
        );
    }
    intersections
}

fn parser<'a>() -> impl Parser<&'a str, Output = [Wire; 2]> {
    let dir = choice((
        token('L').map(|_| Direction::Horizontal(HDirection::Left)),
        token('R').map(|_| Direction::Horizontal(HDirection::Right)),
        token('U').map(|_| Direction::Vertical(VDirection::Up)),
        token('D').map(|_| Direction::Vertical(VDirection::Down)),
    ));
    let segment = dir.followed_by(parser::i32()).map(|(dir, len)| Segment { dir, len });
    let wire = segment.sep_by(comma(), |iter| {
        let mut h_segments = Vec::new();
        let mut v_segments = Vec::new();
        let mut dist = 0;
        let mut pos = Position { x: 0, y: 0 };
        for Segment { dir, len } in iter {
            let Position { x: start_x, y: start_y } = pos;
            match dir {
                Direction::Horizontal(h_dir) => {
                    let (min_x, max_x) = match h_dir {
                        HDirection::Left => {
                            pos.x -= len;
                            (pos.x, start_x)
                        }
                        HDirection::Right => {
                            pos.x += len;
                            (start_x, pos.x)
                        }
                    };
                    h_segments.push(HSegment { min_x, max_x, start_x, y: pos.y, dist });
                }
                Direction::Vertical(v_dir) => {
                    let (min_y, max_y) = match v_dir {
                        VDirection::Up => {
                            pos.y += len;
                            (start_y, pos.y)
                        }
                        VDirection::Down => {
                            pos.y -= len;
                            (pos.y, start_y)
                        }
                    };
                    v_segments.push(VSegment { min_y, max_y, start_y, x: pos.x, dist });
                }
            }
            dist += len;
        }
        h_segments.sort_by_key(|s| s.min_x);
        v_segments.sort_by_key(|s| s.x);
        Some(Wire { h_segments, v_segments })
    });
    chain((wire, newline(), wire)).map(|(a, _, b)| [a, b])
}

pub fn solve(input: &str) -> (i32, i32) {
    let [wire_a, wire_b] = parser().parse_to_end(input).unwrap();
    let mut vec = intersections(&wire_a.h_segments, &wire_b.v_segments);
    vec.extend(intersections(&wire_b.h_segments, &wire_a.v_segments));
    (vec.iter().map(|&(x, _)| x).min().unwrap(), vec.iter().map(|&(_, x)| x).min().unwrap())
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 3).await?;
    assert_eq!(solve(&input), (1983, 107_754));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 3)).unwrap();
        b.iter(|| solve(&input));
    }
}
