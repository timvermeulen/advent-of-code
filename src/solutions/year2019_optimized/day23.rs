use super::*;
use fast_intcode::*;

pub fn solve(input: &str) -> (i64, i64) {
    let memory = intcode::parser().parse_to_end(&input).unwrap();
    let mut queues: Vec<_> = (0..50).map(|_| VecDeque::new()).collect();
    let mut comps: Vec<_> = (0..50).map(|_| Computer::new(memory.to_owned())).collect();

    for i in 0..50 {
        comps[i].step_with(i as i64);
    }

    let mut part1 = None;
    let mut last_y = None;
    let mut last = None;

    loop {
        let mut received_input = false;

        for i in 0..50 {
            let comp = &mut comps[i];

            loop {
                let interrupt = match queues[i].pop_back() {
                    None => comp.step_with(-1),
                    Some((x, y)) => {
                        received_input = true;
                        comp.step_with(x);
                        comp.step_with(y)
                    }
                };

                let index = match interrupt.output() {
                    None => break,
                    Some(i) => i as usize,
                };

                let x = comp.step().unwrap();
                let y = comp.step().unwrap();

                if index == 255 {
                    part1 = part1.or(Some(y));
                    last = Some((x, y));
                } else {
                    queues[index].push_front((x, y));
                }
            }
        }

        if !received_input {
            let (x, y) = last.unwrap();
            queues[0].push_front((x, y));
            if last_y == Some(y) {
                return (part1.unwrap(), y);
            }
            last_y = Some(y);
        }
    }
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 23).await?;
    assert_eq!(solve(&input), (23_213, 17_874));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 23)).unwrap();
        b.iter(|| solve(&input));
    }
}
