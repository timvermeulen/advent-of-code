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

fn part2(range: Range<u32>) -> u32 {
    let range = range.start as usize..range.end as usize;
    let digits = [
        range.start / 100_000,
        (range.start / 10_000) % 10,
        (range.start / 1000) % 10,
        (range.start / 100) % 10,
        (range.start / 10) % 10,
        range.start % 10,
    ];

    let mut n = range.end;
    let mut count = 0;
    let mut freq = [0; 10];
    let mut flag = true;
    for a in digits[0]..10 {
        let diff = a * 100_000;
        if n < diff {
            break;
        }
        n -= diff;
        freq[a] += 1;
        for b in cmp::max(a, if flag { digits[1] } else { 0 })..10 {
            let diff = b * 10_000;
            if n < diff {
                break;
            }
            n -= diff;
            freq[b] += 1;
            for c in cmp::max(b, if flag { digits[2] } else { 0 })..10 {
                let diff = c * 1000;
                if n < diff {
                    break;
                }
                n -= diff;
                freq[c] += 1;
                for d in cmp::max(c, if flag { digits[3] } else { 0 })..10 {
                    let diff = d * 100;
                    if n < diff {
                        break;
                    }
                    n -= diff;
                    freq[d] += 1;
                    for e in cmp::max(d, if flag { digits[4] } else { 0 })..10 {
                        let diff = e * 10;
                        if n < diff {
                            break;
                        }
                        n -= diff;
                        freq[e] += 1;
                        for f in cmp::max(e, if flag { digits[5] } else { 0 })..10 {
                            let diff = f;
                            if n < diff {
                                break;
                            }
                            n -= diff;
                            freq[f] += 1;
                            if freq.contains(&2) {
                                count += 1;
                            }
                            n += diff;
                            freq[f] -= 1;
                        }
                        n += diff;
                        freq[e] -= 1;
                        flag = false;
                    }
                    n += diff;
                    freq[d] -= 1;
                }
                n += diff;
                freq[c] -= 1;
            }
            n += diff;
            freq[b] -= 1;
        }
        n += diff;
        freq[a] -= 1;
    }
    count
}

pub fn solve(input: &str) -> (u32, u32) {
    let range = parser().parse_to_end(&input).unwrap();
    (part1(range.clone()), part2(range))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 4).await?;
    assert_eq!(solve(&input), (1767, 1192));
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
        b.iter(|| solve(&input));
    }
}
