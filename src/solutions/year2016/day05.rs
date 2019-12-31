use crypto::{digest::Digest, md5::Md5};

pub fn solve(input: &str) -> ([u8; 8], [u8; 8]) {
    (part1(input), part2(input))
}

pub fn part1(input: &str) -> [u8; 8] {
    let mut password = [0; 8];
    let mut start = 0;
    for index in 0..8 {
        let (i, c) = find_hash_1(input, start);
        start = i + 1;
        password[index] = c as u8;
    }
    password
}

fn find_hash_1(door_id: &str, lower_bound: usize) -> (usize, char) {
    let mut hasher = Md5::new();

    let hash = |n: usize| -> Option<(usize, char)> {
        hasher.input_str(door_id);
        hasher.input(n.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);
        hasher.reset();

        if output[0] == 0 && output[1] == 0 && output[2] >> 4 == 0 {
            Some((n, std::char::from_digit(output[2] as u32, 16).unwrap()))
        } else {
            None
        }
    };

    (lower_bound..).find_map(hash).unwrap()
}

pub fn part2(input: &str) -> [u8; 8] {
    let mut password = [0; 8];
    let mut start = 0;
    loop {
        let (index, i, c) = find_hash_2(input, start);
        start = i + 1;
        if password[index] == 0 {
            password[index] = c as u8;
            if password.iter().all(|&b| b != 0) {
                for i in 0..8 {
                    dbg!(password[i] as char);
                }
                break;
            }
        }
    }
    password
}

fn find_hash_2(door_id: &str, lower_bound: usize) -> (usize, usize, char) {
    let mut hasher = Md5::new();

    let hash = |n: usize| -> Option<(usize, usize, char)> {
        hasher.input_str(door_id);
        hasher.input(n.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);
        hasher.reset();

        if output[0] == 0 && output[1] == 0 && output[2] >> 4 == 0 && output[2] < 8 {
            let index = output[2] as usize;
            let c = std::char::from_digit((output[3] >> 4) as u32, 16).unwrap();
            Some((index, n, c))
        } else {
            None
        }
    };

    (lower_bound..).find_map(hash).unwrap()
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 5).await?;
    assert_eq!(solve(&input), (*b"f77a0e6e", *b"999828ec"));
    Ok(())
}
