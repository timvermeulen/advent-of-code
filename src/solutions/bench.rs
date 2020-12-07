// #[cfg(test)]
// mod benches {
//     extern crate test;

//     use super::super::*;
//     use test::Bencher;

//     #[bench]
//     fn bench_2019(b: &mut Bencher) {
//         use year2019 as slow;
//         use year2019_optimized as fast;

//         let input = |i| futures::executor::block_on(get_input(2019, i)).unwrap();

//         let input1 = input(1);
//         let input2 = input(2);
//         let input3 = input(3);
//         let input4 = input(4);
//         let input5 = input(5);
//         let input6 = input(6);
//         let input7 = input(7);
//         let input8 = input(8);
//         let input9 = input(9);
//         let input10 = input(10);
//         let input11 = input(11);
//         let input12 = input(12);
//         let input13 = input(13);
//         let input14 = input(14);
//         let input15 = input(15);
//         let input16 = input(16);
//         let input17 = input(17);
//         let input18 = input(18);
//         let input19 = input(19);
//         let input20 = input(20);
//         let input21 = input(21);
//         let input22 = input(22);
//         let input23 = input(23);
//         let input24 = input(24);
//         let input25 = input(25);

//         b.iter(|| {
//             (
//                 slow::day01::solve(&input1),
//                 fast::day02::solve(&input2),
//                 fast::day03::solve(&input3),
//                 fast::day04::solve(&input4),
//                 fast::day05::solve(&input5),
//                 fast::day06::solve(&input6),
//                 fast::day07::solve(&input7),
//                 fast::day08::solve(&input8),
//                 fast::day09::solve(&input9),
//                 fast::day10::solve(&input10),
//                 fast::day11::solve(&input11),
//                 fast::day12::solve(&input12),
//                 fast::day13::solve(&input13),
//                 slow::day14::solve(&input14),
//                 fast::day15::solve(&input15),
//                 fast::day16::solve(&input16),
//                 fast::day17::solve(&input17),
//                 slow::day18::solve_part1(&input18),
//                 fast::day18::part2(&input18),
//                 fast::day19::solve(&input19),
//                 fast::day20::solve(&input20),
//                 fast::day21::solve(&input21),
//                 slow::day22::solve(&input22),
//                 fast::day23::solve(&input23),
//                 fast::day24::solve(&input24),
//                 fast::day25::solve(&input25),
//             )
//         });
//     }
// }

// #[test]
// fn test_intcode() {
//     use super::year2019_optimized::*;
//     let input = |i| futures::executor::block_on(super::get_input(2019, i)).unwrap();

//     (
//         day02::solve(&input(2)),
//         day05::solve(&input(5)),
//         day07::solve(&input(7)),
//         day09::solve(&input(9)),
//         day11::solve(&input(11)),
//         day13::solve(&input(13)),
//         day15::solve(&input(15)),
//         day17::solve(&input(17)),
//         day19::solve(&input(19)),
//         day21::solve(&input(21)),
//         day23::solve(&input(23)),
//         day25::solve(&input(25)),
//     );
// }
