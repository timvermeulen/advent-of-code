use super::*;

pub fn part2(input: &str) -> i32 {
    let width = input.bytes().position(|b| b == b'\n').unwrap();
    let entrance = Pos { x: width as i32 / 2, y: width as i32 / 2 };

    let char_at = |Pos { x, y }| input.as_bytes()[x as usize + y as usize * (width + 1)];
    let is_wall = |pos| char_at(pos) == b'#' || pos.manhattan_distance(entrance) == 1;

    let mut part2 = 0;
    let mut stack = Vec::new();

    for &(dx, dy) in &[(-1, -1), (-1, 1), (1, -1), (1, 1)] {
        let entrance = Pos { x: entrance.x + dx, y: entrance.y + dy };

        let mut max_dist = 0;
        let mut prev_dist = 0;
        let mut last_key_dist = 0;
        let mut visited = 0;

        stack.push((entrance, if dx > 0 { Dir::East } else { Dir::West }, 1));
        while let Some((pos, dir, dist)) = stack.pop() {
            let branch = dist - 1;
            if branch != prev_dist {
                visited -= prev_dist - cmp::max(last_key_dist, branch);
                last_key_dist = cmp::min(last_key_dist, branch);
            }

            prev_dist = dist;
            visited += 1;

            if let b'a'..=b'z' = char_at(pos) {
                max_dist = cmp::max(max_dist, dist);
                last_key_dist = dist;
            }

            let neighbor = pos.moving_to(dir);
            if !is_wall(neighbor) {
                stack.push((neighbor, dir, dist + 1));
            }

            let neighbor = pos.moving_to(dir.left());
            if !is_wall(neighbor) {
                stack.push((neighbor, dir.left(), dist + 1));
            }

            let neighbor = pos.moving_to(dir.right());
            if !is_wall(neighbor) {
                stack.push((neighbor, dir.right(), dist + 1));
            }
        }

        visited -= prev_dist - last_key_dist;
        part2 += visited * 2 - max_dist - 1;
    }

    part2
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 18).await?;
    assert_eq!(part2(&input), 2144);
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 18)).unwrap();
        b.iter(|| part2(&input));
    }
}
