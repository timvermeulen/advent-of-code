use super::*;

#[derive(Copy, Clone)]
struct Item<'a> {
    name: &'a str,
    count: u32,
}

#[derive(Copy, Clone)]
struct Sue<'a> {
    number: u32,
    items: [Item<'a>; 3],
}

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<Sue<'a>>> {
    let item =
        chain((satisfy(char::is_alphabetic).skip_many1().recognize(), string(": "), parser::u32()))
            .map(|(name, _, count)| Item { name, count });
    let items = item.sep_by(string(", "), |iter| Some([iter.next()?, iter.next()?, iter.next()?]));
    let sue = chain((string("Sue "), parser::u32(), string(": "), items))
        .map(|(_, number, _, items)| Sue { number, items });
    sue.collect_sep_by(token('\n'))
}

fn part1(sues: &[Sue<'_>]) -> u32 {
    sues.iter().find(|sue| sue.items.iter().copied().all(is_match_1)).unwrap().number
}

fn is_match_1(item: Item<'_>) -> bool {
    item.count
        == match item.name {
            "children" => 3,
            "cats" => 7,
            "samoyeds" => 2,
            "pomeranians" => 3,
            "akitas" => 0,
            "vizslas" => 0,
            "goldfish" => 5,
            "trees" => 3,
            "cars" => 2,
            "perfumes" => 1,
            _ => unreachable!(),
        }
}

fn part2(sues: &[Sue<'_>]) -> u32 {
    sues.iter().find(|sue| sue.items.iter().copied().all(is_match_2)).unwrap().number
}

fn is_match_2(item: Item<'_>) -> bool {
    match item.name {
        "children" => item.count == 3,
        "cats" => item.count > 7,
        "samoyeds" => item.count == 2,
        "pomeranians" => item.count < 3,
        "akitas" => item.count == 0,
        "vizslas" => item.count == 0,
        "goldfish" => item.count < 5,
        "trees" => item.count > 3,
        "cars" => item.count == 2,
        "perfumes" => item.count == 1,
        _ => unreachable!(),
    }
}

pub fn solve(input: &str) -> (u32, u32) {
    let sues = parser().parse_to_end(&input).unwrap();
    (part1(&sues), part2(&sues))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 16).await?;
    let sues = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&sues), 373);
    assert_eq!(part2(&sues), 260);
    Ok(())
}
