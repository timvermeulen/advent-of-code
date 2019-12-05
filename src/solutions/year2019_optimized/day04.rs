use super::*;

// see https://en.wikipedia.org/wiki/Stars_and_bars_(combinatorics)
fn up_to(n: u32) -> u32 {
    let mut num_steps = 15;
    let mut num_digits = 6;
    let mut total = 0;
    let mut repeated = false;
    let mut last = None;

    for digit in digits_len(n, 6) {
        num_steps -= 1;
        num_digits -= 1;

        if let Some(last) = last {
            if last > digit {
                break;
            } else if last == digit {
                repeated = true;
            }
        }

        for d in last.unwrap_or(0)..digit {
            total += combinations(num_steps, num_digits);
            let repeated = repeated || last == Some(d);
            if !repeated {
                total -= combinations(num_steps - num_digits, num_digits);
            }
            num_steps -= 1;
        }

        last = Some(digit);
    }

    total
}

// see https://en.wikipedia.org/wiki/Combination
fn combinations(n: u32, k: u32) -> u32 {
    if k <= n {
        (1..=k).fold(1, |p, x| p * (n - k + x) / x)
    } else {
        0
    }
}

fn parser<'a>() -> impl Parser<&'a str, Output = Range<u32>> {
    chain((parser::u32(), token('-'), parser::u32())).map(|(a, _, b)| a..b)
}

fn part1(range: Range<u32>) -> u32 {
    up_to(range.end) - up_to(range.start)
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 4).await?;
    let range = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(range), 1767);
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 4)).unwrap();
        b.iter(|| {
            let range = parser().parse_to_end(&input).unwrap();
            part1(range)
        });
    }
}
