pub fn solve(input: &str) -> (u32, u32) {
    let target = input.parse::<u32>().unwrap();
    (part1(target), part2(target))
}

fn part1(target: u32) -> u32 {
    for i in (0..).step_by(2 * 2 * 2 * 2 * 3 * 3 * 5).skip(1) {
        let mut divisor_sum = 1;
        let mut rem = i;

        for div in 2.. {
            if rem == 1 {
                break;
            }
            let mut sum = 1;
            while rem % div == 0 {
                rem /= div;
                sum *= div;
                sum += 1;
            }
            divisor_sum *= sum;
        }

        if divisor_sum * 10 >= target {
            return i;
        }
    }

    unreachable!()
}

fn part2(target: u32) -> u32 {
    for i in (0..).step_by(2 * 2 * 5).skip(1) {
        let mut sum = 0;
        for div in 1..=50 {
            if i % div != 0 {
                continue;
            }
            sum += i / div * 11;
        }
        if sum >= target {
            return i;
        }
    }

    unreachable!()
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 20).await?;
    assert_eq!(solve(&input), (665_280, 705_600));
    Ok(())
}
