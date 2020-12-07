use super::*;
use mask::*;

const P1_NEIGHBORS: [Mask<u32>; 25] = [
    Mask(0b_00000_00000_00000_00001_00010),
    Mask(0b_00000_00000_00000_00010_00101),
    Mask(0b_00000_00000_00000_00100_01010),
    Mask(0b_00000_00000_00000_01000_10100),
    Mask(0b_00000_00000_00000_10000_01000),
    Mask(0b_00000_00000_00001_00010_00001),
    Mask(0b_00000_00000_00010_00101_00010),
    Mask(0b_00000_00000_00100_01010_00100),
    Mask(0b_00000_00000_01000_10100_01000),
    Mask(0b_00000_00000_10000_01000_10000),
    Mask(0b_00000_00001_00010_00001_00000),
    Mask(0b_00000_00010_00101_00010_00000),
    Mask(0b_00000_00100_01010_00100_00000),
    Mask(0b_00000_01000_10100_01000_00000),
    Mask(0b_00000_10000_01000_10000_00000),
    Mask(0b_00001_00010_00001_00000_00000),
    Mask(0b_00010_00101_00010_00000_00000),
    Mask(0b_00100_01010_00100_00000_00000),
    Mask(0b_01000_10100_01000_00000_00000),
    Mask(0b_10000_01000_10000_00000_00000),
    Mask(0b_00010_00001_00000_00000_00000),
    Mask(0b_00101_00010_00000_00000_00000),
    Mask(0b_01010_00100_00000_00000_00000),
    Mask(0b_10100_01000_00000_00000_00000),
    Mask(0b_01000_10000_00000_00000_00000),
];

/// part 2 bitmask layout:
///      |                                     |
///      |                  40                 |
///      |                                     |
/// -----+-----+-----+--------------+-----+----+-----
///      |     |     |              |     |    |
///      |  0  |  1  |       2      |  3  |  4 |
///      |     |     |              |     |    |
///      +-----|-----+--------------+-----+----+
///      |     |     |              |     |    |
///      | 15  | 20  |      16      | 21  |  5 |
///      |     |     |              |     |    |
///      +-----|-----+--------------+-----+----+
///      |     |     |24|25|26|27|28|     |    |
///      |     |     |--+--+--+--+--|     |    |
///      |     |     |39|  |  |  |29|     |    |
///      |     |     |--+--+--+--+--|     |    |
///  43  | 14  | 19  |38|  |  |  |30|  17 |  6 |  41
///      |     |     |--+--+--+--+--|     |    |
///      |     |     |37|  |  |  |31|     |    |
///      |     |     |--+--+--+--+--|     |    |
///      |     |     |36|35|34|33|32|     |    |
///      +-----|-----+--------------+-----+----+
///      |     |     |              |     |    |
///      | 13  | 23  |      18      | 22  |  7 |
///      |     |     |              |     |    |
///      +-----|-----+--------------+-----+----+
///      |     |     |              |     |    |
///      | 12  | 11  |      10      |  9  |  8 |
///      |     |     |              |     |    |
/// -----+-----+-----+--------------+-----+----+-----
///      |                                     |
///      |                  42                 |
///      |                                     |
///
const P2_NEIGHBORS: [Mask<u32>; 44] = [
    // 0-15
    Mask(0b_0000_0000_1000000000000010),
    Mask(0b_0001_0000_0000000000000101),
    Mask(0b_0000_0001_0000000000001010),
    Mask(0b_0010_0000_0000000000010100),
    Mask(0b_0000_0000_0000000000101000),
    Mask(0b_0010_0000_0000000001010000),
    Mask(0b_0000_0010_0000000010100000),
    Mask(0b_0100_0000_0000000101000000),
    Mask(0b_0000_0000_0000001010000000),
    Mask(0b_0100_0000_0000010100000000),
    Mask(0b_0000_0100_0000101000000000),
    Mask(0b_1000_0000_0001010000000000),
    Mask(0b_0000_0000_0010100000000000),
    Mask(0b_1000_0000_0101000000000000),
    Mask(0b_0000_1000_1010000000000000),
    Mask(0b_0001_0000_0100000000000001),
    // 16-19
    Mask(0b_0011_0000_0000000000000100),
    Mask(0b_0110_0000_0000000001000000),
    Mask(0b_1100_0000_0000010000000000),
    Mask(0b_1001_0000_0100000000000000),
    // 20-23
    Mask(0b_0000_1001_1000000000000010),
    Mask(0b_0000_0011_0000000000101000),
    Mask(0b_0000_0110_0000001010000000),
    Mask(0b_0000_1100_0010100000000000),
    // 24-39
    Mask(0b_0000_1001_0000000000000000),
    Mask(0b_0000_0001_0000000000000000),
    Mask(0b_0000_0001_0000000000000000),
    Mask(0b_0000_0001_0000000000000000),
    Mask(0b_0000_0011_0000000000000000),
    Mask(0b_0000_0010_0000000000000000),
    Mask(0b_0000_0010_0000000000000000),
    Mask(0b_0000_0010_0000000000000000),
    Mask(0b_0000_0110_0000000000000000),
    Mask(0b_0000_0100_0000000000000000),
    Mask(0b_0000_0100_0000000000000000),
    Mask(0b_0000_0100_0000000000000000),
    Mask(0b_0000_1100_0000000000000000),
    Mask(0b_0000_1000_0000000000000000),
    Mask(0b_0000_1000_0000000000000000),
    Mask(0b_0000_1000_0000000000000000),
    // 40-43
    Mask(0b_0000_0000_0000000000011111),
    Mask(0b_0000_0000_0000000111110000),
    Mask(0b_0000_0000_0001111100000000),
    Mask(0b_0000_0000_1111000000000001),
];

fn part1(input: &str) -> u32 {
    let mut mask = p1_parse(input);
    let mut seen = Vec::new();

    loop {
        if seen.iter().any(|&m| m == mask) {
            return mask.0;
        } else {
            seen.push(mask);
            mask = p1_evolve(mask);
        }
    }
}

fn p1_parse(input: &str) -> Mask<u32> {
    let mut mask = Mask::empty();
    for y in 0..5 {
        for x in 0..5 {
            if input.as_bytes()[6 * y + x] == b'#' {
                mask.insert((5 * y + x) as u32);
            }
        }
    }
    mask
}

fn p1_evolve(mask: Mask<u32>) -> Mask<u32> {
    let mut one = Mask::empty();
    let mut two = Mask::empty();
    let mut three = Mask::empty();

    for i in mask.iter() {
        let n = P1_NEIGHBORS[i as usize];
        three |= two & n;
        two |= one & n;
        one |= n;
    }

    one & !((!mask | two) & (mask | three))
}

fn part2(input: &str) -> u32 {
    let mask = p2_parse(input);
    let mut layers = [Mask::<u64>::empty(); 201];
    layers[100] = mask;

    for minute in 0..200 {
        let delta = minute / 2 + 1;
        for i in 100 - delta..100 + delta {
            layers[i] |= (layers[i + 1] & Mask((1 << 16) - 1)) << 24;
            layers[i + 1] |= (layers[i] & Mask((1 << 20) - (1 << 16))) << 24;
        }
        for layer in &mut layers[100 - delta..=100 + delta] {
            *layer = p2_evolve(*layer);
        }
    }

    layers
        .iter()
        .map(|&m| (m.0 & ((1 << 24) - 1)).count_ones())
        .sum()
}

fn p2_parse(input: &str) -> Mask<u64> {
    let byte_indices = [
        0, 1, 2, 3, 4, 10, 16, 22, 28, 27, 26, 25, 24, 18, 12, 6, 8, 15, 20, 13, 7, 9, 21, 19,
    ];

    let mut mask = Mask::empty();
    for (i, &byte_index) in (0..).zip(&byte_indices) {
        if input.as_bytes()[byte_index] == b'#' {
            mask.insert(i);
        }
    }
    mask
}

fn p2_evolve(mask: Mask<u64>) -> Mask<u64> {
    let mut one = Mask::empty();
    let mut two = Mask::empty();
    let mut three = Mask::empty();

    for i in mask.iter() {
        let n = P2_NEIGHBORS[i as usize];
        three |= two & n;
        two |= one & n;
        one |= n;
    }

    let one = Mask::<u64>::from(one);
    let two = Mask::<u64>::from(two);
    let three = Mask::<u64>::from(three);

    one & !((!mask | two) & (mask | three))
}

pub fn solve(input: &str) -> (u32, u32) {
    (part1(input), part2(input))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 24).await?;
    assert_eq!(part1(&input), 28_615_131);
    assert_eq!(part2(&input), 1926);
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 24)).unwrap();
        b.iter(|| (part1(&input), part2(&input)));
    }
}
