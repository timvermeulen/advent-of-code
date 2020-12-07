pub fn solve(input: &str) -> (usize, usize) {
    (
        input.lines().filter(|s| supports_tls(s)).count(),
        input.lines().filter(|s| supports_ssl(s)).count(),
    )
}

fn supports_tls(s: &str) -> bool {
    let mut supports_tls = false;
    for (i, part) in s.split(|c| ['[', ']'].contains(&c)).enumerate() {
        if i % 2 == 1 {
            if is_abba(part) {
                return false;
            }
        } else if !supports_tls && is_abba(part) {
            supports_tls = true;
        }
    }
    supports_tls
}

fn supports_ssl(s: &str) -> bool {
    let mut abas = [(false, false); 26 * 26];
    let index_of = |a: u8, b: u8| (a - b'a') as usize * 26 + (b - b'a') as usize;

    for (i, part) in s.split(|c| ['[', ']'].contains(&c)).enumerate() {
        let triples = part
            .as_bytes()
            .windows(3)
            .map(|w| [w[0], w[1], w[2]])
            .filter(|[a, b, c]| a == c && a != b);

        if i % 2 == 0 {
            for [a, b, _] in triples {
                let i = index_of(a, b);
                if abas[i].1 {
                    return true;
                }
                abas[i].0 = true;
            }
        } else {
            for [a, b, _] in triples {
                let i = index_of(b, a);
                if abas[i].0 {
                    return true;
                }
                abas[i].1 = true;
            }
        }
    }
    false
}

fn is_abba(s: &str) -> bool {
    s.as_bytes()
        .windows(4)
        .any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
}

#[cfg(test)]
use super::*;

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 7).await?;
    assert_eq!(solve(&input), (110, 242));
    Ok(())
}
