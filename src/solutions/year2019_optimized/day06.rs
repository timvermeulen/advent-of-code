use super::*;

pub fn solve(input: &str) -> (usize, usize) {
    input.len();
    let num_objects = (input.len() + 1) / 8 + 1;
    let mut index = 0;
    let mut indices = HashMap::new();
    let mut index_of = |object| match indices.get(&object) {
        None => {
            index += 1;
            indices.insert(object, index);
            index
        }
        Some(&i) => i,
    };
    let mut orbits = vec![0; num_objects + 1];
    let mut orbit_counts = vec![0; num_objects + 1];
    for i in 0..num_objects - 1 {
        let f = |j| input.as_bytes()[i * 8 + j];
        let parent = [f(0), f(1), f(2)];
        let child = [f(4), f(5), f(6)];
        let i_p = index_of(parent);
        let i_c = index_of(child);
        orbits[i_c] = i_p;
        orbit_counts[i_p] += 1;
    }

    let mut objects: Vec<_> = (1..num_objects + 1).filter(|&i| orbit_counts[i] == 0).collect();
    let mut part1 = 0;
    let mut path_counts = vec![0; num_objects + 1];
    while let Some(object) = objects.pop() {
        let path_count = path_counts[object];
        let parent = orbits[object];
        part1 += path_count;
        if parent == 0 {
            break;
        }
        path_counts[parent] += path_count + 1;
        if orbit_counts[parent] == 1 {
            objects.push(parent);
        } else {
            orbit_counts[parent] -= 1;
        }
    }

    let mut distances = vec![0; num_objects + 1];

    let mut index = index_of([b'Y', b'O', b'U']);
    let mut dist = 0;
    while index != 0 {
        let parent = orbits[index];
        distances[index] = dist;
        dist += 1;
        index = parent;
    }

    let mut index = index_of([b'S', b'A', b'N']);
    let mut dist = 0;
    loop {
        let parent = orbits[index];
        if distances[parent] != 0 {
            let part2 = distances[parent] + dist - 1;
            return (part1, part2);
        }
        dist += 1;
        index = parent;
    }
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2019, 6).await?;
    assert_eq!(solve(&input), (227_612, 454));
    Ok(())
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = futures::executor::block_on(get_input(2019, 6)).unwrap();
        b.iter(|| solve(&input));
    }
}
