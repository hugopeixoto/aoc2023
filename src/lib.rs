#![feature(test)]
extern crate test;

pub mod brainfuck;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;

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
        paste::paste! {
            #[bench]
            fn [<bench_ $day>](b: &mut test::Bencher) {
                let filename = format!("inputs/{}.in", stringify!($day));
                let input = std::fs::read_to_string(filename).unwrap();

                b.iter(|| aoc2023::$day::solve(&input));
            }
        }
    }
}
