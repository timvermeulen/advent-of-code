use super::*;

pub fn solve(input: &str) -> (u32, u32) {
    let mut part1 = 0;
    let mut part2 = 0;

    for line in ascii_split(input, b'\n') {
        let bytes = line.as_bytes();

        let read_single = |i: usize| unsafe { *bytes.get_unchecked(i) } - b'0';
        let read_double = |i: usize| 10 * read_single(i) + read_single(i + 1);

        let from;
        let to;
        let offset;

        if bytes[1] == b'-' {
            from = read_single(0);

            if bytes[3] == b' ' {
                to = read_single(2);
                offset = 4;
            } else {
                to = read_double(2);
                offset = 5;
            }
        } else {
            from = read_double(0);

            if bytes[4] == b' ' {
                to = read_single(3);
                offset = 5;
            } else {
                to = read_double(3);
                offset = 6;
            }
        }

        let letter = bytes[offset];

        let count = bytes[(offset + 3)..]
            .iter()
            .filter(|&&c| c == letter)
            .count() as u8;

        let is_match = |k: u8| unsafe { *bytes.get_unchecked(offset + k as usize + 2) } == letter;

        part1 += (count >= from && count <= to) as u32;
        part2 += (is_match(from) != is_match(to)) as u32;
    }

    (part1, part2)
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2020, 2).await?;
    assert_eq!(solve(&input), (560, 303));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2020, 2)).unwrap();
        b.iter(|| solve(&input));
    }
}
