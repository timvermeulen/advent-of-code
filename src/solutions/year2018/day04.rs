use super::*;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
struct Timestamp {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

#[derive(Debug)]
enum Action {
    Begin(u32),
    Awake,
    Asleep,
}

#[derive(Debug)]
struct Record {
    timestamp: Timestamp,
    action: Action,
}

#[derive(Copy, Clone)]
struct MinutesAsleep {
    total: u32,
    per_minute: [u32; 60],
}

impl Default for MinutesAsleep {
    fn default() -> Self {
        Self {
            total: 0,
            per_minute: [0; 60],
        }
    }
}

impl MinutesAsleep {
    fn add(&mut self, range: Range<u32>) {
        self.total += range.len() as u32;
        for minute in range {
            self.per_minute[minute as usize] += 1;
        }
    }

    fn most_common_minute(self) -> (u32, u32) {
        (0..60)
            .map(|m| (m, self.per_minute[m as usize]))
            .max_by_key(|&(_, x)| x)
            .unwrap()
    }
}

fn parse(input: &str) -> Vec<Record> {
    let date = chain((
        parser::u32(),
        token('-'),
        parser::u32(),
        token('-'),
        parser::u32(),
    ));
    let time = chain((parser::u32(), token(':'), parser::u32()));
    let timestamp = chain((token('['), date, token(' '), time, token(']'))).map(
        |(_, (year, _, month, _, day), _, (hour, _, minute), _)| Timestamp {
            year,
            month,
            day,
            hour,
            minute,
        },
    );

    let begin = chain((string(" Guard #"), parser::u32(), string(" begins shift")))
        .map(|(_, id, _)| Action::Begin(id));
    let asleep = string(" falls asleep").map(|_| Action::Asleep);
    let awake = string(" wakes up").map(|_| Action::Awake);
    let action = choice((begin.attempt(), asleep.attempt(), awake));

    let record = chain((timestamp, action)).map(|(timestamp, action)| Record { timestamp, action });
    let mut records: Vec<_> = record
        .collect_sep_by(token('\n'))
        .parse_to_end(input)
        .expect("parsing failed");
    records.sort_by_key(|r| r.timestamp);
    records
}

fn tally(records: &[Record]) -> HashMap<u32, MinutesAsleep> {
    let mut map = HashMap::<_, MinutesAsleep>::new();
    let mut current = 0;
    let mut asleep = 0;
    for record in records {
        let minute = record.timestamp.minute;
        match record.action {
            Action::Begin(id) => current = id,
            Action::Asleep => asleep = minute,
            Action::Awake => map.entry(current).or_default().add(asleep..minute),
        }
    }
    map
}

fn part1(records: &[Record]) -> u32 {
    let asleep = tally(records);
    let (id, minutes) = asleep
        .into_iter()
        .max_by_key(|(_, minutes)| minutes.total)
        .expect("the map can't be empty");
    id * minutes.most_common_minute().0
}

fn part2(records: &[Record]) -> u32 {
    let asleep = tally(records);
    let (id, (minute, _)) = asleep
        .into_iter()
        .map(|(id, minutes)| (id, minutes.most_common_minute()))
        .max_by_key(|(_, x)| x.1)
        .unwrap();
    id * minute
}

pub fn solve(input: &str) -> (u32, u32) {
    let records = parse(&input);
    (part1(&records), part2(&records))
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2018, 4).await?;
    let records = parse(&input);
    assert_eq!(part1(&records), 30_630);
    assert_eq!(part2(&records), 136_571);
    Ok(())
}
