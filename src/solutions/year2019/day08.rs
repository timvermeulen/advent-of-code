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

fn part2(input: &str) -> String {
    let mut pixels = [' '; WIDTH * HEIGHT];
    for layer in input.as_bytes().rchunks(SIZE) {
        for (pixel, digit) in pixels.iter_mut().zip(layer) {
            match digit {
                b'0' => *pixel = ' ',
                b'1' => *pixel = '*',
                _ => {}
            }
        }
    }
    pixels.chunks(WIDTH).flat_map(|row| row.iter().copied().chain(iter::once('\n'))).collect()
}

const FHJUL: &str = "\
**** *  *   ** *  * *    
*    *  *    * *  * *    
***  ****    * *  * *    
*    *  *    * *  * *    
*    *  * *  * *  * *    
*    *  *  **   **  **** 
";

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 8).await?;
    assert_eq!(part1(&input), 2250);
    assert_eq!(part2(&input), FHJUL);
    Ok(())
}
