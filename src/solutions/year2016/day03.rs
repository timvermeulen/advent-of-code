use super::*;

fn is_triangle([x, y, z]: [u32; 3]) -> bool {
    x + y > z && y + z > x && z + x > y
}

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<[u32; 3]>> {
    let number = chain((token(' ').skip_many(), parser::u32())).map(|((), x)| x);
    let triple = chain((number, number, number)).map(|(x, y, z)| [x, y, z]);
    triple.collect_sep_by(token('\n'))
}

fn part1(triples: &[[u32; 3]]) -> u32 {
    triples.iter().filter(|&&t| is_triangle(t)).count() as u32
}

fn part2(triples: &[[u32; 3]]) -> u32 {
    let mut iter = triples.iter().copied();
    std::iter::from_fn(|| try { [iter.next()?, iter.next()?, iter.next()?] })
        .flat_map(|[[a, b, c], [d, e, f], [g, h, i]]| {
            std::array::IntoIter::new([[a, d, g], [b, e, h], [c, f, i]])
        })
        .filter(|&t| is_triangle(t))
        .count() as u32
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 3).await?;
    let triples = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&triples), 982);
    assert_eq!(part2(&triples), 1826);
    Ok(())
}
