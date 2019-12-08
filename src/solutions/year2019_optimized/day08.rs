use super::*;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const SIZE: usize = WIDTH * HEIGHT;

fn part1(input: &str) -> usize {
    let layer = input.as_bytes().chunks(SIZE).min_by_key(|l| count(l, b'0')).unwrap();
    count(layer, b'1') * count(layer, b'2')
}

fn count(layer: &[u8], digit: u8) -> usize {
    layer.iter().filter(|&&x| x == digit).count()
}

fn part2(input: &str) -> [u8; 5] {
    let mut pixels = [[false; 5]; 5];
    for layer in input.as_bytes().rchunks(SIZE) {
        for (pixels, &offset) in pixels.iter_mut().zip(&[0, 5, 10, 15, 20]) {
            for (pixel, &index) in pixels.iter_mut().zip(&[2, 3, 76, 78, 128]) {
                match layer[index + offset] {
                    b'0' => *pixel = false,
                    b'1' => *pixel = true,
                    _ => {}
                }
            }
        }
    }
    [
        identify(pixels[0]),
        identify(pixels[1]),
        identify(pixels[2]),
        identify(pixels[3]),
        identify(pixels[4]),
    ]
}

const CHARS: [u8; 32] = [
    b'Y', b'C', b' ', b'F', b' ', b'P', b' ', b' ', b' ', b'B', b'U', b'J', b' ', b' ', b' ', b' ',
    b'L', b' ', b'K', b'E', b' ', b'R', b' ', b'Z', b' ', b'G', b'H', b' ', b' ', b'A', b' ', b' ',
];

fn identify([a, b, c, d, e]: [bool; 5]) -> u8 {
    CHARS[a as usize + 2 * b as usize + 4 * c as usize + 8 * d as usize + 16 * e as usize]
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 8).await?;
    assert_eq!(part1(&input), 2250);
    assert_eq!(part2(&input), *b"FHJUL");
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 8)).unwrap();
        b.iter(|| part1(&input));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 8)).unwrap();
        b.iter(|| part2(&input));
    }
}
