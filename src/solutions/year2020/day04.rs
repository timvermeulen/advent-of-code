#[cfg(test)]
use super::*;

pub fn solve(input: &str) -> (u32, u32) {
    let mut part1 = 0;
    let mut part2 = 0;

    for passport in input.split("\n\n") {
        let mut fields: u8 = 0;
        let mut valid = true;

        for field in passport.split_whitespace() {
            let bytes = field.as_bytes();
            let get = |i: usize| unsafe { *bytes.get_unchecked(i) };
            let get_digit = |i: usize| (get(i) - b'0') as u32;

            let (a, b, c) = (get(0), get(1), get(2));

            if c == b'r' {
                // byr/iyr/eyr

                let (bit, lo, hi) = match a {
                    b'b' => (0, 1920, 2002),
                    b'i' => (1, 2010, 2020),
                    b'e' => (2, 2020, 2030),
                    _ => unreachable!(),
                };

                fields |= 1 << bit;

                let year =
                    1000 * get_digit(4) + 100 * get_digit(5) + 10 * get_digit(6) + get_digit(7);
                if year < lo || year > hi {
                    valid = false;
                }
            } else if b == b'g' {
                // hgt
                fields |= 1 << 3;

                if bytes.len() < 7 || get(7) != b'c' && get(7) != b'n' {
                    valid = false;
                }
            } else if a == b'h' {
                // hcl
                fields |= 1 << 4;

                if bytes.len() != 11 {
                    valid = false;
                }
            } else if b == b'c' {
                // ecl
                fields |= 1 << 5;

                const VALID: &[&[u8]] = &[b"amb", b"blu", b"brn", b"gry", b"grn", b"hzl", b"oth"];

                if !VALID.contains(&&bytes[4..]) {
                    valid = false;
                }
            } else if a == b'p' {
                // pid
                fields |= 1 << 6;

                if bytes.len() != 13 {
                    valid = false;
                }
            }
        }

        if fields == 0b0111_1111 {
            part1 += 1;

            if valid {
                part2 += 1;
            }
        }
    }

    (part1, part2)
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2020, 4).await?;
    assert_eq!(solve(&input), (230, 156));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2020, 4)).unwrap();
        b.iter(|| solve(&input));
    }
}
