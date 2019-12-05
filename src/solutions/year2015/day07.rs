use super::*;

type Wire = String;
type Value = u16;

#[derive(Debug)]
enum SingleSource {
    W(Wire),
    V(Value),
}

impl SingleSource {
    fn eval(&self, f: &mut impl FnMut(&Wire) -> Value) -> Value {
        match self {
            SingleSource::W(w) => f(w),
            SingleSource::V(v) => *v,
        }
    }
}

#[derive(Debug)]
enum Gate {
    And(SingleSource, SingleSource),
    Or(SingleSource, SingleSource),
    Lshift(SingleSource, Value),
    Rshift(SingleSource, Value),
    Not(SingleSource),
}

impl Gate {
    fn eval(&self, f: &mut impl FnMut(&Wire) -> Value) -> Value {
        let mut f = |s: &SingleSource| s.eval(f);
        match self {
            Gate::And(a, b) => f(a) & f(b),
            Gate::Or(a, b) => f(a) | f(b),
            Gate::Lshift(a, n) => f(a) << n,
            Gate::Rshift(a, n) => f(a) >> n,
            Gate::Not(a) => !f(a),
        }
    }
}

#[derive(Debug)]
enum Source {
    Gate(Gate),
    SingleSource(SingleSource),
}

impl Source {
    fn eval(&self, f: &mut impl FnMut(&Wire) -> Value) -> Value {
        match self {
            Source::Gate(g) => g.eval(f),
            Source::SingleSource(s) => s.eval(f),
        }
    }
}

fn parser<'a>() -> impl Parser<&'a str, Output = HashMap<Wire, Source>> {
    let value = parser::u16();
    let wire = satisfy(char::is_alphabetic).skip_many1().recognize().map(str::to_string);

    let s = wire.map(SingleSource::W).or(value.map(SingleSource::V));

    let gate = choice((
        chain((s, string(" AND "), s)).map(|(a, _, b)| Gate::And(a, b)).attempt(),
        chain((s, string(" OR "), s)).map(|(a, _, b)| Gate::Or(a, b)).attempt(),
        chain((s, string(" LSHIFT "), value)).map(|(a, _, b)| Gate::Lshift(a, b)).attempt(),
        chain((s, string(" RSHIFT "), value)).map(|(a, _, b)| Gate::Rshift(a, b)).attempt(),
        chain((string("NOT "), s)).map(|(_, a)| Gate::Not(a)).attempt(),
    ));

    let source = gate.map(Source::Gate).or(s.map(Source::SingleSource));
    let instruction = chain((source, string(" -> "), wire)).map(|(s, _, w)| (w, s));

    instruction.collect_sep_by(token('\n'))
}

fn part1(instructions: &HashMap<Wire, Source>) -> Value {
    let mut cache = Cache::new(|wire, f| {
        let s = instructions.get(wire).unwrap();
        s.eval(&mut |w| f(w.clone()))
    });
    cache.get("a".into())
}

fn part2(instructions: &HashMap<Wire, Source>) -> Value {
    let mut cache = Cache::new(|wire, f| {
        let s = instructions.get(wire).unwrap();
        s.eval(&mut |w| f(w.clone()))
    });
    cache.insert("b".into(), part1(instructions));
    cache.get("a".into())
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2015, 7).await?;
    let instructions = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&instructions), 46_065);
    assert_eq!(part2(&instructions), 14_134);
    Ok(())
}
