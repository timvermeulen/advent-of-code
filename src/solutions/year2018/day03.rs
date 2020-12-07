use super::*;

#[derive(Copy, Clone)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn overlaps(self: Claim, other: Claim) -> bool {
        fn overlaps_1d(start_0: u32, len_0: u32, start_1: u32, len_1: u32) -> bool {
            (start_0 >= start_1 && start_0 < start_1 + len_1)
                || (start_1 >= start_0 && start_1 < start_0 + len_0)
        }
        overlaps_1d(self.x, self.width, other.x, other.width)
            && overlaps_1d(self.y, self.height, other.y, other.height)
    }
}

fn parse(input: &str) -> Vec<Claim> {
    let id = chain((token('#'), parser::u32()));
    let origin = chain((parser::u32(), token(','), parser::u32()));
    let size = chain((parser::u32(), token('x'), parser::u32()));
    let claim = chain((id, string(" @ "), origin, string(": "), size)).map(
        |((_, id), _, (x, _, y), _, (width, _, height))| Claim {
            id,
            x,
            y,
            width,
            height,
        },
    );
    claim
        .collect_sep_by(token('\n'))
        .parse_to_end(&input)
        .unwrap()
}

fn part1(claims: &[Claim]) -> u32 {
    let mut counts = vec![0; 1_000_000];
    for &claim in claims {
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                let i = (1000 * x + y) as usize;
                counts[i] += 1;
            }
        }
    }
    counts.iter().filter(|&&c| c > 1).count() as u32
}

fn part2(claims: &[Claim]) -> u32 {
    let (_, claim) = claims
        .iter()
        .enumerate()
        .find(|&(i, &first)| {
            claims
                .iter()
                .enumerate()
                .all(|(j, &second)| i == j || !first.overlaps(second))
        })
        .unwrap();
    claim.id
}

pub fn solve(input: &str) -> (u32, u32) {
    let claims = parse(input);
    (part1(&claims), part2(&claims))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2018, 3).await?;
    let claims = parse(&input);
    assert_eq!(part1(&claims), 110_389);
    assert_eq!(part2(&claims), 552);
    Ok(())
}
