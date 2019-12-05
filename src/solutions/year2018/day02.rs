use super::*;

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<&'a str>> {
    satisfy(char::is_alphabetic).skip_many1().recognize().collect_sep_by(token('\n'))
}

fn part1(ids: &[&str]) -> u32 {
    let mut two_count = 0;
    let mut three_count = 0;
    ids.iter().copied().for_each(|id| {
        let mut map = HashMap::new();
        id.chars().for_each(|c| {
            map.entry(c).and_modify(|k| *k += 1).or_insert(1);
        });
        let contains = |n| map.values().any(|&k| k == n);
        two_count += contains(2) as u32;
        three_count += contains(3) as u32;
    });
    two_count * three_count
}

fn part2(ids: &[&str]) -> String {
    let mut pairs =
        ids.iter().enumerate().flat_map(|(i, &a)| ids[i + 1..].iter().map(move |&b| (a, b)));
    let (a, b) = pairs
        .find(|(a, b)| {
            let chars = || a.chars().zip(b.chars());
            chars().filter(|(a, b)| a != b).count() == 1
        })
        .unwrap();
    a.chars().zip(b.chars()).filter(|(a, b)| a == b).map(|(a, _)| a).collect()
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2018, 2).await?;
    let ids = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&ids), 7163);
    assert_eq!(part2(&ids), "ighfbyijnoumxjlxevacpwqtr");
    Ok(())
}
