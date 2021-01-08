use super::*;

#[derive(Copy, Clone)]
struct Destination {
    is_bot: bool,
    number: u32,
}

#[derive(Default)]
struct State {
    connections: HashMap<u32, (Destination, Destination)>,
    bots: HashMap<u32, u32>,
    outputs: HashMap<u32, u32>,
}

impl State {
    fn add_to_bot(&mut self, value: u32, bot: u32) -> Option<u32> {
        let existing = match self.bots.get(&bot) {
            Some(&existing) => existing,
            None => {
                self.bots.insert(bot, value);
                return None;
            }
        };

        let lo = cmp::min(value, existing);
        let hi = cmp::max(value, existing);

        let (lo_dest, hi_dest) = self.connections[&bot];

        if let bot @ Some(_) = self
            .send_to_dest(lo, lo_dest)
            .or(self.send_to_dest(hi, hi_dest))
        {
            bot
        } else if lo == 17 && hi == 61 {
            Some(bot)
        } else {
            None
        }
    }

    fn send_to_dest(&mut self, value: u32, dest: Destination) -> Option<u32> {
        if dest.is_bot {
            self.add_to_bot(value, dest.number)
        } else {
            self.outputs.insert(dest.number, value);
            None
        }
    }
}

pub fn solve(input: &str) -> (u32, u32) {
    let mut state = State::default();
    let mut initial_values = Vec::<(u32, u32)>::new();

    for line in input.lines() {
        let mut words = line.split_whitespace();

        let is_bot = |word: Option<&str>| word.unwrap() == "bot";
        let int = |word: Option<&str>| word.unwrap().parse::<u32>().unwrap();

        if is_bot(words.next()) {
            let from = int(words.next());

            let lo = Destination {
                is_bot: is_bot(words.nth(3)),
                number: int(words.next()),
            };

            let hi = Destination {
                is_bot: is_bot(words.nth(3)),
                number: int(words.next()),
            };

            state.connections.insert(from, (lo, hi));
        } else {
            initial_values.push((int(words.next()), int(words.nth(3))));
        }
    }

    let mut part1 = 0;

    for (value, bot) in initial_values {
        if let Some(x) = state.add_to_bot(value, bot) {
            part1 = x;
        }
    }

    let part2 = state.outputs[&0] * state.outputs[&1] * state.outputs[&2];
    (part1, part2)
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 10).await?;
    assert_eq!(solve(&input), (56, 7847));
    Ok(())
}
