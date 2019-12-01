use super::*;

#[derive(Debug)]
enum JSON<'a> {
    Array(Vec<JSON<'a>>),
    Object(Vec<(&'a str, JSON<'a>)>),
    Number(i32),
    String(&'a str),
}

impl JSON<'_> {
    fn sum(&self) -> i32 {
        match self {
            JSON::Array(vec) => vec.iter().map(JSON::sum).sum(),
            JSON::Object(vec) => vec.iter().map(|(_, json)| json.sum()).sum(),
            JSON::Number(n) => *n,
            JSON::String(_) => 0,
        }
    }

    fn non_red_sum(&self) -> i32 {
        match self {
            JSON::Array(vec) => vec.iter().map(JSON::non_red_sum).sum(),
            JSON::Object(vec) => {
                if vec.iter().any(|(_, value)| match value {
                    JSON::String(string) => *string == "red",
                    _ => false,
                }) {
                    0
                } else {
                    vec.iter().map(|(_, json)| json.non_red_sum()).sum()
                }
            }
            JSON::Number(n) => *n,
            JSON::String(_) => 0,
        }
    }
}

#[opaque]
fn json<'a>() -> impl Parser<&'a str, Output = JSON<'a>> {
    choice((
        parser::i32().map(JSON::Number),
        string().map(JSON::String),
        array().map(JSON::Array),
        object().map(JSON::Object),
    ))
}

fn string<'a>() -> impl Parser<&'a str, Output = &'a str> {
    satisfy(char::is_alphabetic).skip_many1().recognize().between(token('\"'), token('\"'))
}

fn array<'a>() -> impl Parser<&'a str, Output = Vec<JSON<'a>>> {
    json().collect_sep_by(token(',')).between(token('['), token(']'))
}

fn object<'a>() -> impl Parser<&'a str, Output = Vec<(&'a str, JSON<'a>)>> {
    chain((string(), token(':'), json()))
        .map(|(key, _, value)| (key, value))
        .collect_sep_by(token(','))
        .between(token('{'), token('}'))
}

fn parse(input: &str) -> JSON<'_> {
    json().parse_to_end(input).unwrap()
}

fn part1(input: &str) -> i32 {
    parse(input).sum()
}

fn part2(input: &str) -> i32 {
    parse(input).non_red_sum()
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 12).await?;
    assert_eq!(part1(&input), 156_366);
    assert_eq!(part2(&input), 96_852);
    Ok(())
}
