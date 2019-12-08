use super::*;
use intcode::*;
use permutohedron::Heap;

fn parser<'a>() -> impl Parser<&'a str, Output = Vec<i32>> {
    parser::i32().collect_sep_by(comma())
}

fn part1(memory: &[i32]) -> i32 {
    let mut settings = [0, 1, 2, 3, 4];
    let mut heap = Heap::new(&mut settings);
    let iter = iter::from_fn(|| {
        let p = heap.next_permutation()?;
        Some([p[0], p[1], p[2], p[3], p[4]])
    });
    iter.map(|settings| run_part1(memory, settings)).max().unwrap()
}

fn run_part1(memory: &[i32], settings: [i32; 5]) -> i32 {
    let amp = || Computer::new(memory.to_owned());
    let mut amps = [amp(), amp(), amp(), amp(), amp()];
    iter!(amps).zip(&settings).fold(0, |input, (mut amp, &setting)| {
        amp.run();
        amp.run_with_input(Some(setting));
        amp.run_with_input(Some(input)).output().unwrap()
    })
}

fn part2(memory: &[i32]) -> i32 {
    let mut settings = [5, 6, 7, 8, 9];
    let mut heap = Heap::new(&mut settings);
    let iter = iter::from_fn(|| {
        let p = heap.next_permutation()?;
        Some([p[0], p[1], p[2], p[3], p[4]])
    });
    iter.map(|settings| run_part2(memory, settings)).max().unwrap()
}

fn run_part2(memory: &[i32], settings: [i32; 5]) -> i32 {
    let amp = || Computer::new(memory.to_owned());
    let mut amps = [amp(), amp(), amp(), amp(), amp()];

    for i in 0..5 {
        amps[i].run();
        amps[i].run_with_input(Some(settings[i]));
    }

    let mut value = 0;

    loop {
        let mut halted = false;
        for amp in &mut amps {
            value = amp.run_with_input(Some(value)).output().unwrap();
            if let State::Halt = amp.run() {
                halted = true;
            }
        }
        if halted {
            return value;
        }
    }
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 7).await?;
    let memory = parser().parse_to_end(&input).unwrap();
    assert_eq!(part1(&memory), 212_460);
    assert_eq!(part2(&memory), 21_844_737);
    Ok(())
}
