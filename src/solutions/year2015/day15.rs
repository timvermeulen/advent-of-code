use parser::prelude::*;
use std::cmp;

pub fn solve(input: &str) -> (i32, i32) {
    let garbage = parser::satisfy(|c: char| c != '-' && !c.is_numeric()).skip_many();
    let property = garbage.followed_by(parser::i32()).map(|(_, x)| x);
    let ingredient = property.many(|iter| {
        Some([
            iter.next()?,
            iter.next()?,
            iter.next()?,
            iter.next()?,
            iter.next()?,
        ])
    });
    let properties: [[i32; 5]; 4] = ingredient
        .sep_by(token('\n'), |iter| {
            Some([iter.next()?, iter.next()?, iter.next()?, iter.next()?])
        })
        .parse_to_end(input)
        .unwrap();

    let mut part1 = 0;
    let mut part2 = 0;

    for i in 0..=100 {
        for j in 0..=(100 - i) {
            for k in 0..=(100 - i - j) {
                let l = 100 - i - j - k;

                let score = |index| {
                    let s = |i: usize| properties[i][index];
                    cmp::max(0, i * s(0) + j * s(1) + k * s(2) + l * s(3))
                };

                let total = score(0) * score(1) * score(2) * score(3);
                part1 = cmp::max(part1, total);

                if score(4) == 500 {
                    part2 = cmp::max(part2, total);
                }
            }
        }
    }

    (part1, part2)
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 15).await?;
    assert_eq!(solve(&input), (21_367_368, 1_766_400));
    Ok(())
}
