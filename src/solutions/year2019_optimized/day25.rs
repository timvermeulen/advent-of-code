use super::*;
use fast_intcode::*;

#[derive(Clone)]
struct Droid {
    comp: Computer,
}

impl Dir {
    fn as_string(self) -> &'static str {
        match self {
            Self::North => "north",
            Self::South => "south",
            Self::West => "west",
            Self::East => "east",
        }
    }
}

impl Droid {
    fn new(memory: Vec<i64>) -> Self {
        Self { comp: Computer::new(memory) }
    }

    fn log_and_move_to(&mut self, dir: Dir) -> String {
        self.log_and_run_with(dir.as_string())
    }

    fn move_to(&mut self, dir: Dir) {
        self.run_with_and_ignore_output(dir.as_string());
    }

    fn take(&mut self, item: &str) {
        self.enter("take ");
        self.run_with_and_ignore_output(item);
    }

    fn drop(&mut self, item: &str) {
        self.enter("drop ");
        self.run_with_and_ignore_output(item);
    }

    fn enter(&mut self, input: &str) {
        self.comp.step_with_iter(input.bytes().map(|b| b as i64));
    }

    fn run(&mut self) -> String {
        self.comp.run().map(|x| x as u8 as char).collect()
    }

    fn run_with(&mut self, input: &str) -> impl Iterator<Item = i64> + '_ {
        self.enter(input);
        self.comp.run_with(b'\n' as i64)
    }

    fn run_with_and_ignore_output(&mut self, input: &str) {
        self.run_with(input).for_each(drop);
    }

    fn log_and_run_with(&mut self, input: &str) -> String {
        self.run_with(input).map(|x| x as u8 as char).collect()
    }
}

// fn log<T>(label: &str, f: impl FnOnce() -> T) -> T {
//     use std::time::Instant;
//     let start = Instant::now();
//     let res = f();
//     println!("{}, time spent: {:?}", label, start.elapsed());
//     res
// }

pub fn solve(input: &str) -> u32 {
    let memory = intcode::parser().parse_to_end(&input).unwrap();
    let mut droid = Droid::new(memory);

    let mut stack = vec![(None::<Dir>, 0)];
    let mut moves = Vec::new();
    let mut moves_to_checkpoint = None;
    let mut items = Vec::new();

    let forbidden =
        ["infinite loop", "photons", "molten lava", "escape pod", "giant electromagnet"];

    while let Some((dir, dist)) = stack.pop() {
        if dist < moves.len() {
            // backtracking
            moves.drain(dist..).rev().for_each(|dir: Dir| {
                droid.move_to(dir.opposite());
            })
        }

        let output = match dir {
            None => droid.run(),
            Some(dir) => {
                moves.push(dir);
                droid.log_and_move_to(dir)
            }
        };

        let back = dir.map(Dir::opposite);

        if output.starts_with("\n\n\n== Security") {
            let final_dir = Dir::all()
                .find(|&dir| back != Some(dir) && output.contains(dir.as_string()))
                .unwrap();
            moves_to_checkpoint = Some((moves.clone(), final_dir));
            continue;
        }

        let phrase = "Items here:\n";
        if let Some(i) = output.find(phrase) {
            let item: String = output
                .bytes()
                .skip(i + phrase.len() + 2)
                .take_while(|&b| b != b'\n')
                .map(|b| b as char)
                .collect();
            if !forbidden.iter().any(|&x| x == item) {
                droid.take(&item);
                items.push(item);
            }
        }

        for next_dir in Dir::all() {
            if back != Some(next_dir) && output.contains(next_dir.as_string()) {
                stack.push((Some(next_dir), moves.len()));
            }
        }
    }

    // go back to the start
    moves.into_iter().rev().for_each(|dir| {
        droid.move_to(dir.opposite());
    });

    // go to the checkpoint
    let (moves, dir) = moves_to_checkpoint.unwrap();
    moves.into_iter().for_each(|dir| {
        droid.move_to(dir);
    });

    #[derive(Debug)]
    enum TestResult {
        Passcode(u32),
        TooHeavy,
        TooLight,
    }

    let attempt = |droid: &mut Droid| {
        let output = droid.log_and_move_to(dir);
        if output.ends_with("Command?\n") {
            if output.contains("heavier") {
                TestResult::TooLight
            } else {
                TestResult::TooHeavy
            }
        } else {
            let passcode = output
                .chars()
                .skip_while(|&c| !c.is_digit(10))
                .take_while(|&c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .fold(0, |n, d| 10 * n + d);
            TestResult::Passcode(passcode)
        }
    };

    items.sort_by_key(|item| item.len());

    let mut no_items = droid.clone();
    for item in &items {
        no_items.drop(item);
    }

    loop {
        let mut required = [false; 8];
        for i in 0..items.len() {
            let mut clone = droid.clone();
            clone.drop(&items[i]);
            match attempt(&mut clone) {
                TestResult::Passcode(code) => return code,
                TestResult::TooHeavy => {}
                TestResult::TooLight => {
                    no_items.take(&items[i]);
                    required[i] = true;
                }
            }
        }
        for i in (0..8).rev() {
            if required[i] {
                items.swap_remove(i);
            }
        }

        let mut too_heavy = [false; 8];
        for i in 0..items.len() {
            let mut clone = no_items.clone();
            clone.take(&items[i]);
            match attempt(&mut clone) {
                TestResult::Passcode(code) => return code,
                TestResult::TooHeavy => {
                    droid.drop(&items[i]);
                    too_heavy[i] = true;
                }
                TestResult::TooLight => {}
            }
        }
        for i in (0..8).rev() {
            if too_heavy[i] {
                items.swap_remove(i);
            }
        }
    }
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 25).await?;
    assert_eq!(solve(&input), 285_278_336);
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 25)).unwrap();
        b.iter(|| solve(&input));
    }
}
