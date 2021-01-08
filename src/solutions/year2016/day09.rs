fn decompressed_len(bytes: &[u8], recursive: bool) -> usize {
    let mut i = 0;
    let mut length = 0;

    let read_int = |i: &mut usize| {
        let mut n = 0;
        while (b'0'..=b'9').contains(&bytes[*i]) {
            n *= 10;
            n += (bytes[*i] - b'0') as usize;
            *i += 1;
        }
        n
    };

    while let Some(j) = bytes[i..].iter().position(|&b| b == b'(') {
        length += j;
        i += j;
        i += 1;
        let len = read_int(&mut i);
        i += 1;
        let times = read_int(&mut i);
        i += 1;

        length += if recursive {
            decompressed_len(&bytes[i..][..len], true)
        } else {
            len
        } * times;
        i += len;
    }

    length + (bytes.len() - i)
}

pub fn solve(input: &str) -> (usize, usize) {
    (
        decompressed_len(input.as_bytes(), false),
        decompressed_len(input.as_bytes(), true),
    )
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 9).await?;
    assert_eq!(solve(&input), (120_765, 11_658_395_076));
    Ok(())
}
