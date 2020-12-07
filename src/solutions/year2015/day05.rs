fn part1(input: &str) -> usize {
    input.lines().filter(|&s| is_nice1(s)).count()
}

fn is_nice1(string: &str) -> bool {
    string.chars().filter(|&c| "aeiou".contains(c)).count() >= 3
        && string
            .chars()
            .zip(string.chars().skip(1))
            .any(|(a, b)| a == b)
        && !["ab", "cd", "pq", "xy"].iter().any(|&s| string.contains(s))
}

fn part2(input: &str) -> usize {
    input.lines().filter(|&s| is_nice2(s)).count()
}

fn is_nice2(string: &str) -> bool {
    rule1(string) && rule2(string)
}

fn rule1(string: &str) -> bool {
    let bytes = string.as_bytes();
    (0..bytes.len() - 3).any(|i| {
        bytes[i + 2..]
            .windows(2)
            .any(|s| s[0] == bytes[i] && s[1] == bytes[i + 1])
    })
}

fn rule2(string: &str) -> bool {
    string
        .chars()
        .zip(string.chars().skip(2))
        .any(|(a, b)| a == b)
}

pub fn solve(input: &str) -> (usize, usize) {
    (part1(input), part2(input))
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 5).await?;
    assert_eq!(part1(&input), 258);
    assert_eq!(part2(&input), 53);
    Ok(())
}
