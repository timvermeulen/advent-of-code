pub fn solve(input: &str) -> (usize, usize) {
    let n: usize = input.parse().unwrap();
    let mut vec = vec![true; n];

    let next = |i: usize, table: &[bool]| {
        if let Some(j) = table[(i + 1)..].iter().position(|&b| b) {
            i + j + 1
        } else {
            table.iter().position(|&b| b).unwrap()
        }
    };

    let mut a = 0;
    let mut b = 1;

    for _ in 0..(n - 1) {
        vec[b] = false;
        a = next(b, &vec);
        b = next(a, &vec);
    }

    let part1 = a + 1;

    vec = vec![true; n];

    let mut a = 0;
    let mut b = n / 2;

    for remaining in (2..=n).rev() {
        vec[b] = false;
        a = next(a, &vec);
        b = next(b, &vec);

        if remaining % 2 == 1 {
            b = next(b, &vec);
        }
    }

    let part2 = a + 1;
    (part1, part2)
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 19).await?;
    assert_eq!(solve(&input), (1_808_357, 1_407_007));
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
