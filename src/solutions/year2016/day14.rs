use crypto::{digest::Digest, md5::Md5};
use std::{cmp, collections::VecDeque};

fn solve_(input: &str, stretched: bool) -> u32 {
    let mut triples = VecDeque::<(u8, u32)>::new();
    let mut keys = Vec::new();
    let mut max_key = 0;

    for index in 0.. {
        if let Some(&(_, i)) = triples.front() {
            if i + 1000 < index {
                triples.pop_front();
            }

            if keys.len() >= 64 && i > max_key {
                break;
            }
        }

        let hash = hash(input, index, stretched);

        triples.retain(|&(b, i)| {
            if hash.windows(5).any(|w| w.iter().all(|&x| b == x)) {
                keys.push(i);
                max_key = cmp::max(max_key, i);
                false
            } else {
                true
            }
        });

        if let Some(&[a, _, _]) = hash.array_windows().find(|[a, b, c]| a == b && b == c) {
            triples.push_back((a, index));
        }
    }

    keys.sort_unstable();
    keys[63]
}

pub fn solve(input: &str) -> (u32, u32) {
    (solve_(input, false), solve_(input, true))
}

fn hash(input: &str, index: u32, stretched: bool) -> [u8; 32] {
    let mut hasher = Md5::new();

    hasher.input_str(input);
    hasher.input_str(&index.to_string());

    if stretched {
        for _ in 0..2016 {
            let string = hasher.result_str();
            hasher.reset();
            hasher.input_str(&string);
        }
    }

    let mut hash = [0; 16];
    hasher.result(&mut hash);

    let mut output = [0; 32];
    for ([hi, lo], byte) in output.array_chunks_mut().zip(&hash) {
        *hi = byte >> 4;
        *lo = byte & 0x0f;
    }
    output
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 14).await?;
    assert_eq!(solve(&input), (15_035, 19_968));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2016, 14)).unwrap();
        b.iter(|| solve(&input));
    }
}
