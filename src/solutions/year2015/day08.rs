use super::*;

#[derive(Debug, Copy, Clone)]
enum EscapeSequence {
    Backslash,
    Quote,
    Character,
}

impl EscapeSequence {
    fn code(self) -> u32 {
        match self {
            EscapeSequence::Backslash | EscapeSequence::Quote => 2,
            EscapeSequence::Character => 4,
        }
    }

    fn encoded(self) -> u32 {
        match self {
            EscapeSequence::Backslash | EscapeSequence::Quote => 4,
            EscapeSequence::Character => 5,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Token {
    Escape(EscapeSequence),
    Letters(u32),
}

impl Token {
    fn code(self) -> u32 {
        match self {
            Token::Escape(s) => s.code(),
            Token::Letters(c) => c,
        }
    }

    fn memory(self) -> u32 {
        match self {
            Token::Escape(_) => 1,
            Token::Letters(c) => c,
        }
    }

    fn encoded(self) -> u32 {
        match self {
            Token::Escape(s) => s.encoded(),
            Token::Letters(c) => c,
        }
    }
}

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<Vec<Token>>> {
    let hex_digit = satisfy_map(|c: char| c.to_digit(16));
    let word = satisfy(char::is_alphabetic).many1(|iter| Some(iter.count() as u32));

    let escaped = choice((
        token('\\').map(|_| EscapeSequence::Backslash),
        token('\"').map(|_| EscapeSequence::Quote),
        chain((token('x'), hex_digit, hex_digit)).map(|_| EscapeSequence::Character),
    ));

    let escape = token('\\').followed_by(escaped).map(|(_, s)| s);
    let tokens = escape
        .map(Token::Escape)
        .or(word.map(Token::Letters))
        .collect_many();

    let literal = tokens.between(token('\"'), token('\"'));
    let literals = literal.collect_sep_by(token('\n'));
    literals
}

fn part1(tokens: &[Vec<Token>]) -> u32 {
    tokens
        .iter()
        .map(|tokens| tokens.iter().fold(2, |n, t| n + t.code() - t.memory()))
        .sum()
}

fn part2(tokens: &[Vec<Token>]) -> u32 {
    tokens
        .iter()
        .map(|tokens| tokens.iter().fold(4, |n, t| n + t.encoded() - t.code()))
        .sum()
}

pub fn solve(input: &str) -> (u32, u32) {
    let tokens = parser().parse_to_end(&input).unwrap();
    (part1(&tokens), part2(&tokens))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 8).await?;
    let tokens = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&tokens), 1333);
    assert_eq!(part2(&tokens), 2046);
    Ok(())
}
