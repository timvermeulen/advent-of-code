use super::*;

#[derive(Hash, Eq, PartialEq)]
struct Amount<'a> {
    amount: i64,
    chemical: Chemical<'a>,
}

#[derive(Hash, Eq, PartialEq)]
struct Reaction<'a> {
    inputs: Vec<Amount<'a>>,
    output: Amount<'a>,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Chemical<'a>(&'a str);

const FUEL: Chemical<'static> = Chemical("FUEL");

fn ore_cost(fuel: i64, reactions: &HashMap<Chemical<'_>, Reaction<'_>>) -> i64 {
    let mut counts = HashMap::new();

    for (_, reaction) in reactions {
        for ingredient in &reaction.inputs {
            *counts.entry(ingredient.chemical).or_insert(0) += 1;
        }
    }

    let mut ready = vec![Amount { amount: fuel, chemical: FUEL }];
    let mut not_ready: HashMap<Chemical<'_>, _> = HashMap::new();

    while let Some(Amount { amount, chemical }) = ready.pop() {
        let reaction = match reactions.get(&chemical) {
            Some(x) => x,
            None => return amount,
        };
        let times = (amount - 1) / reaction.output.amount + 1;

        for input in &reaction.inputs {
            let amount =
                input.amount * times + not_ready.get(&input.chemical).copied().unwrap_or(0);
            let count = counts.get_mut(&input.chemical).unwrap();
            *count -= 1;
            if *count == 0 {
                ready.push(Amount { amount, chemical: input.chemical });
            } else {
                not_ready.insert(input.chemical, amount);
            }
        }
    }

    unreachable!()
}

fn part1(reactions: &HashMap<Chemical<'_>, Reaction<'_>>) -> i64 {
    ore_cost(1, reactions)
}

fn optimal_ore_cost(
    chemical: Chemical<'_>,
    reactions: &HashMap<Chemical<'_>, Reaction<'_>>,
) -> f64 {
    let reaction = match reactions.get(&chemical) {
        None => return 1.0,
        Some(x) => x,
    };
    let x = 1.0 / reaction.output.amount as f64;
    reaction
        .inputs
        .iter()
        .map(|&Amount { amount, chemical }| {
            amount as f64 / reaction.output.amount as f64 * optimal_ore_cost(chemical, reactions)
        })
        .sum()
}

fn part2(reactions: &HashMap<Chemical<'_>, Reaction<'_>>) -> i64 {
    const TRILLION: i64 = 1_000_000_000_000;
    let upper_bound = (TRILLION as f64 / optimal_ore_cost(FUEL, reactions)).floor() as i64;
    (0..=upper_bound).rev().find(|&n| ore_cost(n, reactions) <= TRILLION).unwrap()
}

fn parser<'a>() -> impl Parser<&'a str, Output = HashMap<Chemical<'a>, Reaction<'a>>> {
    let amount = chain((parser::i64(), token(' '), alphabetic().skip_many().recognize()))
        .map(|(amount, _, chemical)| Amount { amount, chemical: Chemical(chemical) });
    let inputs = amount.collect_sep_by(string(", "));
    let reaction = chain((inputs, string(" => "), amount))
        .map(|(inputs, _, output)| Reaction { inputs, output });
    reaction.map(|r: Reaction<'a>| (r.output.chemical, r)).collect_sep_by(newline())
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 14).await?;
    let reactions = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&reactions), 198_984);
    assert_eq!(part2(&reactions), 7_659_732);
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 14)).unwrap();
        b.iter(|| parser().parse_to_end(&input).unwrap());
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 14)).unwrap();
        let reactions = parser().parse_to_end(&input).unwrap();
        b.iter(|| part1(&reactions));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 14)).unwrap();
        let reactions = parser().parse_to_end(&input).unwrap();
        b.iter(|| part2(&reactions));
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 14)).unwrap();
        b.iter(|| {
            let reactions = parser().parse_to_end(&input).unwrap();
            (part1(&reactions), part2(&reactions))
        });
    }
}
