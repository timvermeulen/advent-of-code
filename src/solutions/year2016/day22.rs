#[cfg(test)]
use super::*;

struct Node {
    x: u32,
    y: u32,
    used: u32,
    available: u32,
}

pub fn solve(input: &str) {
    let mut nodes = Vec::new();

    for line in input.lines().skip(2) {
        let mut iter = line
            .split(|c: char| !c.is_digit(10))
            .filter_map(|s| s.parse::<u32>().ok());
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let _size = iter.next().unwrap();
        let used = iter.next().unwrap();
        let available = iter.next().unwrap();

        nodes.push(Node {
            x,
            y,
            used,
            available,
        });
    }

    nodes.sort_by_key(|node| (node.y, node.x));

    let mut count = 0;

    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if i != j && nodes[i].used > 0 && nodes[i].used <= nodes[j].available {
                count += 1;
            }
        }
    }

    println!("part 1: {}", count);

    for node in nodes {
        if node.x == 0 {
            println!();
        }

        if node.used > 100 {
            print!("X");
        } else if node.used == 0 {
            print!("_");
        } else if node.x == 0 && node.y == 0 {
            print!("T");
        } else if node.x == 32 && node.y == 0 {
            print!("O");
        } else {
            print!(".");
        }
    }

    // width: 33, height: 31
    // 3 + 28 + 32 + 5 * 31
}

#[async_std::test]
async fn test() -> Result<(), InputError> {
    let input = get_input(2016, 22).await?;
    solve(&input);
    Ok(())
}

// #[cfg(test)]
// mod benches {
//     extern crate test;

//     use super::*;
//     use test::Bencher;

//     #[bench]
//     fn bench(b: &mut Bencher) {
//         let input = futures::executor::block_on(get_input(2016, 18)).unwrap();
//         b.iter(|| solve(&input));
//     }
// }
