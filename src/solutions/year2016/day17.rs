use crypto::{digest::Digest, md5::Md5};

pub fn solve(input: &str) -> (String, usize) {
    let mut paths = vec![(0, 0, String::new())];
    let mut shortest = None;
    let mut longest = None;

    while !paths.is_empty() {
        let mut new = Vec::new();

        for (x, y, path) in paths {
            if x == 3 && y == 3 {
                if shortest.is_none() {
                    shortest = Some(path.clone());
                }
                longest = Some(path.clone());
                continue;
            }

            let [up, down, left, right] = hash(input, &path);

            if up && y > 0 {
                let mut path = path.clone();
                path.push('U');
                new.push((x, y - 1, path));
            }

            if down && y < 3 {
                let mut path = path.clone();
                path.push('D');
                new.push((x, y + 1, path));
            }

            if left && x > 0 {
                let mut path = path.clone();
                path.push('L');
                new.push((x - 1, y, path));
            }

            if right && x < 3 {
                let mut path = path.clone();
                path.push('R');
                new.push((x + 1, y, path));
            }
        }

        paths = new;
    }

    (shortest.unwrap(), longest.unwrap().len())
}

fn hash(input: &str, path: &str) -> [bool; 4] {
    let mut hasher = Md5::new();

    hasher.input_str(input);
    hasher.input_str(path);
    let mut hash = [0; 16];
    hasher.result(&mut hash);

    let is_open = |x: u8| x > 10;
    [
        is_open(hash[0] >> 4),
        is_open(hash[0] & 0x0f),
        is_open(hash[1] >> 4),
        is_open(hash[1] & 0x0f),
    ]
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 17).await?;
    assert_eq!(solve(&input), ("DRRDRLDURD".to_string(), 618));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2016, 17)).unwrap();
        b.iter(|| solve(&input));
    }
}
