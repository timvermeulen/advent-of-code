pub fn solve(input: &str) -> ([u8; 8], [u8; 8]) {
    let mut password = [0; 8];
    for i in 0..8 {
        password[i] = input.as_bytes()[i];
    }
    let first = password_after(password);
    let second = password_after(first);
    (first, second)
}

fn password_after(mut password: [u8; 8]) -> [u8; 8] {
    loop {
        let mut i = 7;
        while password[i] == b'z' {
            password[i] = b'a';
            i -= 1;
        }
        password[i] += 1;
        if is_valid(password) {
            for &i in &password {
                dbg!(i as char);
            }
            return password;
        }
    }
}

fn is_valid(chars: [u8; 8]) -> bool {
    if chars.iter().any(|c| b"iol".contains(c)) {
        return false;
    }
    if !(0..6).any(|i| chars[i] + 1 == chars[i + 1] && chars[i + 1] + 1 == chars[i + 2]) {
        return false;
    }
    let i = match (0..5).find(|&i| chars[i] == chars[i + 1]) {
        None => return false,
        Some(i) => i,
    };
    if !(i + 2..7).any(|j| chars[i] != chars[j] && chars[j] == chars[j + 1]) {
        return false;
    }
    true
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 11).await?;
    assert_eq!(solve(&input), (*b"cqjxxyzz", *b"cqkaabcc"));
    Ok(())
}
