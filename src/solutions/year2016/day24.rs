use super::*;

pub fn solve(input: &str) -> (u32, u32) {
    let mut path = HashSet::new();
    let mut numbers = [(0, 0); 8];
    let mut map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, &c) in line.as_bytes().iter().enumerate() {
            if c == b'#' {
                continue;
            }

            path.insert((x, y));

            if c != b'.' {
                let n = (c - b'0') as usize;
                numbers[n] = (x, y);
                map.insert((x, y), n);
            }
        }
    }

    let mut seen = HashSet::new();
    let mut nodes = vec![(numbers[0], 0)];

    let mut part1 = None;

    for round in 0.. {
        let mut new = Vec::new();

        for ((x, y), mut mask) in nodes {
            if let Some(&n) = map.get(&(x, y)) {
                mask |= 1 << n;

                if mask == 0b1111_1111 {
                    if part1.is_none() {
                        part1 = Some(round);
                    }

                    if n == 0 {
                        return (part1.unwrap(), round);
                    }
                }
            }

            if !seen.insert(((x, y), mask)) {
                continue;
            }

            for (x, y) in iter!([(x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1)]) {
                if path.contains(&(x, y)) {
                    new.push(((x, y), mask));
                }
            }
        }

        nodes = new;
    }

    unreachable!()
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 24).await?;
    assert_eq!(solve(&input), (470, 720));
    Ok(())
}

// #[cfg(test)]
// mod benches {
//     extern crate test;

//     use super::*;
//     use test::Bencher;

//     #[bench]
//     fn bench(b: &mut Bencher) {
//         let input = futures::executor::block_on(get_input(2016, 18)).unwrap();
//         b.iter(|| solve(&input));
//     }
// }
