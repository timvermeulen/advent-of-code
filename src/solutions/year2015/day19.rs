use super::*;

#[derive(Debug)]
struct Replacement<'a> {
    from: &'a str,
    to: &'a str,
}

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<Replacement<'a>>> {
    let molecule = satisfy(char::is_alphabetic).skip_many().recognize();
    let replacement =
        chain((molecule, string(" => "), molecule)).map(|(from, _, to)| Replacement { from, to });
    replacement.collect_sep_by(token('\n'))
}

pub fn solve(mut input: &str) -> (usize, usize) {
    let replacements = parser().parse(&mut input).unwrap();
    let molecule = input.trim_start();
    let mut set = HashSet::new();

    for r in &replacements {
        set.extend(replacement_results(molecule, r.from, r.to));
    }

    let part1 = set.len();

    let mut string = molecule.to_string();

    for round in 1.. {
        let next = replacements
            .iter()
            .find_map(|r| replacement_results(&string, r.to, r.from).next())
            .unwrap();

        if next == "e" {
            let part2 = round;
            return (part1, part2);
        }
        string = next;
    }

    unreachable!()
}

fn replacement_results<'a>(
    string: &'a str,
    from: &'a str,
    to: &'a str,
) -> impl Iterator<Item = String> + 'a {
    string.match_indices(from).map(move |(i, _)| {
        let mut result = String::new();
        result.push_str(string.get(..i).unwrap());
        result.push_str(to);
        result.push_str(string.get((i + from.len())..).unwrap());
        result
    })
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 19).await?;
    assert_eq!(solve(&input), (535, 212));
    Ok(())
}
