use super::*;

fn parse(input: &str) -> Vec<u32> {
    parser::u32().collect_sep_by(token('\n')).parse_to_end(input).unwrap()
}

fn part1(containers: &[u32]) -> u32 {
    count1(containers, 150)
}

fn count1(containers: &[u32], liters: u32) -> u32 {
    if liters == 0 {
        return 1;
    }

    let (first, rest) = match containers.split_first() {
        Some((&first, rest)) => (first, rest),
        None => return 0,
    };

    if first <= liters {
        count1(rest, liters) + count1(rest, liters - first)
    } else {
        count1(rest, liters)
    }
}

fn part2(containers: &[u32]) -> u32 {
    count2(containers, 150).unwrap().1
}

// -> (container count, combination count)
fn count2(containers: &[u32], liters: u32) -> Option<(u32, u32)> {
    if liters == 0 {
        return Some((0, 1));
    }

    let (first, rest) = match containers.split_first() {
        Some((&first, rest)) => (first, rest),
        None => return None,
    };

    let without = count2(rest, liters);

    if first > liters {
        return without;
    }

    let with = match count2(rest, liters - first).map(|(x, y)| (x + 1, y)) {
        Some(with) => with,
        None => return without,
    };

    let without = match without {
        Some(without) => without,
        None => return Some(with),
    };

    Some(match with.0.cmp(&without.0) {
        Ordering::Less => with,
        Ordering::Equal => (with.0, with.1 + without.1),
        Ordering::Greater => without,
    })
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 17).await?;
    let containers = parse(&input);
    assert_eq!(part1(&containers), 654);
    assert_eq!(part2(&containers), 57);
    Ok(())
}
