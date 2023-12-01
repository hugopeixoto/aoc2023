#![feature(test)]
extern crate test;

pub mod day01;

#[macro_export]
macro_rules! run {
    ( $day:ident ) => {
        use std::fs::read_to_string;

        let filename = format!("inputs/{}.in", stringify!($day));
        let input = std::fs::read_to_string(filename).unwrap();

        let (p1, p2) = aoc2023::$day::solve(&input);

        println!("{}", p1);
        println!("{}", p2);
    };
}

#[macro_export]
macro_rules! bench {

    ( $day:ident ) => {
        use paste::paste;
        paste! {
            #[bench]
            fn [<bench_ $day>](b: &mut test::Bencher) {
                let filename = format!("inputs/{}.in", stringify!($day));
                let input = std::fs::read_to_string(filename).unwrap();

                b.iter(|| aoc2023::$day::solve(&input));
            }
        }
    }
}
