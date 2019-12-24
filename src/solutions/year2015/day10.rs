use super::*;
use itertools::Itertools;

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<u8>> {
    parser::digit().map(|d| d as u8).collect_many()
}

fn look_and_say(slice: &[u8]) -> Vec<u8> {
    slice
        .iter()
        .copied()
        .group_by(|&n| n)
        .into_iter()
        .flat_map(|(k, g)| vec![g.count() as u8, k])
        .collect()
}

fn part1(digits: &[u8]) -> usize {
    let mut vec = digits.to_owned();
    for _ in 0..40 {
        vec = look_and_say(&vec);
    }
    vec.len()
}

fn part2(digits: &[u8]) -> usize {
    let mut vec = digits.to_owned();
    for _ in 0..50 {
        vec = look_and_say(&vec);
    }
    vec.len()
}

pub fn solve(input: &str) -> (usize, usize) {
    let digits = parser().parse_to_end(&input).unwrap();
    (part1(&digits), part2(&digits))
}

#[test]
fn test_example() {
    assert_eq!(look_and_say(&[1]), &[1, 1]);
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 10).await?;
    let digits = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&digits), 360_154);
    assert_eq!(part2(&digits), 5_103_798);
    Ok(())
}
