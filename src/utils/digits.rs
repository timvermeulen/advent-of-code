use std::iter;

fn rev_digits(mut n: u32) -> impl Iterator<Item = u32> {
    iter::from_fn(move || {
        if n == 0 {
            None
        } else {
            let d = n % 10;
            n /= 10;
            Some(d)
        }
    })
}

fn inf_rev_digits(mut n: u32) -> impl Iterator<Item = u32> {
    iter::from_fn(move || {
        let d = n % 10;
        n /= 10;
        Some(d)
    })
}

pub fn digits(n: u32) -> impl Iterator<Item = u32> {
    let (rev, len) = rev_digits(n).fold((0, 0), |(k, len), d| (10 * k + d, len + 1));
    inf_rev_digits(rev).take(len)
}

pub fn digits_len(n: u32, len: u32) -> impl Iterator<Item = u32> {
    let rev = inf_rev_digits(n)
        .take(len as usize)
        .fold(0, |k, d| 10 * k + d);
    inf_rev_digits(rev).take(len as usize)
}
