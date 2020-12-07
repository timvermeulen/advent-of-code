use super::*;
use mod_exp::mod_exp;
use modinverse::modinverse;

#[derive(Debug)]
enum Technique {
    Deal,
    Cut(i128),
    Increment(i128),
}

impl Technique {
    fn apply(&self, index: i128, num_cards: i128) -> i128 {
        match *self {
            Self::Deal => num_cards - 1 - index,
            Self::Cut(n) => (index - n + num_cards) % num_cards,
            Self::Increment(n) => (index * n) % num_cards,
        }
    }

    fn apply_reversed(&self, index: i128, num_cards: i128) -> i128 {
        match *self {
            Self::Deal => num_cards - 1 - index,
            Self::Cut(n) => (index + n + num_cards) % num_cards,
            Self::Increment(n) => {
                let inv = modinverse(n, num_cards).unwrap();
                (index * inv) % num_cards
            }
        }
    }
}

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<Technique>> {
    let deal = string("deal into new stack").map(|_| Technique::Deal);
    let cut = string("cut ")
        .followed_by(parser::i128())
        .map(|(_, n)| Technique::Cut(n));
    let increment = string("deal with increment ")
        .followed_by(parser::i128())
        .map(|(_, n)| Technique::Increment(n));
    choice((deal.attempt(), cut.attempt(), increment)).collect_sep_by(newline())
}

fn part1(techniques: &[Technique]) -> i128 {
    techniques
        .iter()
        .fold(2019, |index, t| t.apply(index, 10_007))
}

fn part2(techniques: &[Technique]) -> i128 {
    const NUM_CARDS: i128 = 119_315_717_514_047;
    const NUM_SHUFFLES: i128 = 101_741_582_076_661;

    let apply = |index| {
        techniques
            .iter()
            .rev()
            .fold(index, |index, t| t.apply_reversed(index, NUM_CARDS))
    };

    let x: i128 = 2020;
    let y = apply(x);
    let z = apply(y);

    let a = (y - z) * modinverse(x - y + NUM_CARDS, NUM_CARDS).unwrap();
    let b = (y - a * x).rem_euclid(NUM_CARDS);

    let t = mod_exp(a, NUM_SHUFFLES, NUM_CARDS) * x;
    let u = (mod_exp(a, NUM_SHUFFLES, NUM_CARDS) - 1).rem_euclid(NUM_CARDS);
    let v = (modinverse(a - 1, NUM_CARDS).unwrap() * b).rem_euclid(NUM_CARDS);

    (t + u * v).rem_euclid(NUM_CARDS)
}

pub fn solve(input: &str) -> (i128, i128) {
    let techniques = parser().parse_to_end(&input).unwrap();
    (part1(&techniques), part2(&techniques))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 22).await?;
    let techniques = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&techniques), 4775);
    assert_eq!(part2(&techniques), 37_889_219_674_304);
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 22)).unwrap();
        b.iter(|| {
            let techniques = parser().parse_to_end(&input).unwrap();
            (part1(&techniques), part2(&techniques))
        });
    }
}
