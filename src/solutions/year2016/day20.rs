use parser::prelude::*;

pub fn solve(input: &str) -> (u32, u32) {
    let mut vec = Vec::new();
    let parser = chain((parser::u32(), token('-'), parser::u32())).map(|(from, _, to)| (from, to));
    for (from, to) in parser.iter_sep_by(token('\n'), input) {
        vec.push((from, 1));
        vec.push((to, -1));
    }
    vec.sort_by_key(|&(x, _)| x);

    let mut lowest_allowed = None;
    let mut num_allowed = 0;

    let mut last = 0;
    let mut count = 0;
    for (x, d) in vec {
        if count == 0 {
            let allowed = last + 1;
            if x > allowed {
                lowest_allowed = lowest_allowed.or(Some(allowed));
                num_allowed += x - allowed;
            }
        }
        count += d;
        if count == 0 {
            last = x;
        }
    }

    (lowest_allowed.unwrap(), num_allowed)
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 20).await?;
    assert_eq!(solve(&input), (14_975_795, 101));
    Ok(())
}
