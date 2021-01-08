fn solve_(input: &str, disk_size: usize) -> String {
    let mut vec = Vec::<bool>::new();

    for &b in input.as_bytes() {
        vec.push(b == b'1');
    }

    while vec.len() < disk_size {
        vec.push(false);

        for i in (0..(vec.len() - 1)).rev().take(disk_size - vec.len()) {
            vec.push(!vec[i]);
        }
    }

    while vec.len() % 2 == 0 {
        vec = vec.array_chunks().map(|[a, b]| a == b).collect();
    }

    vec.into_iter().fold(String::new(), |mut s, b| {
        s.push(if b { '1' } else { '0' });
        s
    })
}

pub fn solve(input: &str) -> (String, String) {
    (solve_(input, 272), solve_(input, 35651584))
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 16).await?;
    assert_eq!(
        solve(&input),
        (
            "10010100110011100".to_string(),
            "01100100101101100".to_string()
        )
    );
    solve(&input);
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2016, 16)).unwrap();
        b.iter(|| solve(&input));
    }
}
