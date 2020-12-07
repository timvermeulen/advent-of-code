use super::*;

fn parser<'a>() -> impl Parser<&'a str, Output = HashMap<&'a str, &'a str>> {
    let planet = parser::any()
        .many(|iter| {
            for _ in 0..3 {
                iter.next()?;
            }
            Some(())
        })
        .recognize();
    let orbit =
        chain((planet, token::<&str>(')'), planet)).map(|(parent, _, child)| (child, parent));
    orbit.collect_sep_by(newline())
}

fn path<'a>(map: &'a HashMap<&'a str, &'a str>, planet: &'a str) -> impl Iterator<Item = &'a str> {
    successors(Some(planet), move |&p| map.get(p).copied())
}

fn part1(orbits: &HashMap<&str, &str>) -> usize {
    orbits
        .values()
        .map(|planet| path(orbits, planet).count())
        .sum()
}

fn part2(orbits: &HashMap<&str, &str>) -> usize {
    let path = |start| path(orbits, start).skip(1).collect::<Vec<_>>();
    let a = path("YOU");
    let b = path("SAN");
    let in_common = a
        .iter()
        .rev()
        .zip(b.iter().rev())
        .take_while(|(a, b)| a == b)
        .count();
    a.len() + b.len() - 2 * in_common
}

pub fn solve(input: &str) -> (usize, usize) {
    let orbits = parser().parse_to_end(&input).unwrap();
    (part1(&orbits), part2(&orbits))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 6).await?;
    assert_eq!(solve(&input), (227_612, 454));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 6)).unwrap();
        b.iter(|| solve(&input));
    }
}
