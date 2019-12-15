use super::*;
use intcode::*;

fn part1(arcade: &mut Computer) -> u32 {
    let mut block_count = 0;

    while let State::Output(_x) = arcade.run() {
        let _y = arcade.run();
        if arcade.run().unwrap() == 2 {
            block_count += 1;
        }
    }

    block_count
}

fn part2(mut arcade: Computer) -> i64 {
    arcade.memory[0] = 2;

    let mut ball_x: i64 = 0;
    let mut paddle_x: i64 = 0;
    let mut output = 0;

    loop {
        let x = match arcade.run_with((ball_x - paddle_x).signum()).output() {
            None => return output,
            Some(x) => x,
        };
        let _y = arcade.run();
        let z = arcade.run().unwrap();

        if x == -1 {
            output = z;
        } else {
            match z {
                3 => paddle_x = x,
                4 => ball_x = x,
                _ => {}
            };
        }
    }
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 13).await?;
    let memory = parser::i64().collect_sep_by(comma()).parse_to_end(&input).unwrap();
    let mut arcade = Computer::new(memory);
    assert_eq!(part1(&mut arcade), 363);
    assert_eq!(part2(arcade), 17_159);
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 13)).unwrap();
        b.iter(|| {
            let memory = parser::i64().collect_sep_by(comma()).parse_to_end(&input).unwrap();
            let mut arcade = Computer::new(memory);
            (part1(&mut arcade), part2(arcade))
        });
    }
}
