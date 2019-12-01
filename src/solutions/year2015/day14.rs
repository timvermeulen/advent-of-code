use super::*;

struct Reindeer {
    speed: u32,
    fly: u32,
    rest: u32,
}

const TIME: u32 = 2503;

impl Reindeer {
    fn distance(&self) -> u32 {
        let period = self.fly + self.rest;
        let count = TIME / period;
        let rem = TIME % period;
        let extra = self.fly.min(rem);
        (count * self.fly + extra) * self.speed
    }

    fn partial_distances(&self) -> impl Iterator<Item = u32> + '_ {
        iter::repeat(self.speed)
            .take(self.fly as usize)
            .chain(iter::repeat(0).take(self.rest as usize))
            .cycle()
            .scan(0, |sum, d| {
                *sum += d;
                Some(*sum)
            })
            .take(TIME as usize)
    }
}

fn parse(input: &str) -> Vec<Reindeer> {
    parser::u32()
        .sep_by(rubbish(), |iter| {
            Some(Reindeer { speed: iter.next()?, fly: iter.next()?, rest: iter.next()? })
        })
        .between(rubbish(), rubbish())
        .collect_sep_by(token('\n'))
        .parse_to_end(input)
        .unwrap()
}

fn part1(reindeer: &[Reindeer]) -> u32 {
    reindeer.iter().map(|s| s.distance()).max().unwrap()
}

fn part2(reindeer: &[Reindeer]) -> u32 {
    let mut scores = vec![0; reindeer.len()];
    let distances: Vec<Vec<u32>> =
        reindeer.iter().map(|reindeer| reindeer.partial_distances().collect()).collect();
    for i in 0..TIME as usize {
        let distances = || distances.iter().map(|d| d[i]);
        let max = distances().max().unwrap();
        for (winner, _) in distances().enumerate().filter(|&(_, d)| d == max) {
            scores[winner] += 1;
        }
    }
    scores.into_iter().max().unwrap()
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 14).await?;
    let reindeer = parse(&input);
    assert_eq!(part1(&reindeer), 2696);
    assert_eq!(part2(&reindeer), 1084);
    Ok(())
}
