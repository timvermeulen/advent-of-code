fn are_opposites(x: char, y: char) -> bool {
    x.eq_ignore_ascii_case(&y) && x != y
}

fn reduced_len(iter: impl Iterator<Item = char>) -> u32 {
    let mut vec = Vec::new();
    for c in iter {
        match vec.last() {
            Some(&last) if are_opposites(c, last) => {
                vec.pop();
            }
            _ => vec.push(c),
        }
    }
    vec.len() as u32
}

fn part1(input: &str) -> u32 {
    reduced_len(input.chars())
}

fn part2(input: &str) -> u32 {
    (b'a'..b'z')
        .map(|c| {
            let c = c as char;
            let filtered = input.chars().filter(|x| !x.eq_ignore_ascii_case(&c));
            reduced_len(filtered)
        })
        .min()
        .unwrap()
}

pub fn solve(input: &str) -> (u32, u32) {
    (part1(input), part2(input))
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2018, 5).await?;
    assert_eq!(part1(&input), 11_668);
    assert_eq!(part2(&input), 4652);
    Ok(())
}
