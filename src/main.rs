#![feature(test)]
#![feature(drain_filter)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

use std::env::args;
use std::fs::read_to_string;
use std::str::FromStr;
use day1::day1;
use day2::day2;
use day3::day3;
use day4::day4;
use day5::day5;
use day6::day6;
use day7::day7;
use day8::day8;
use day9::day9;
use day10::day10;
use day11::day11;


fn main() {
    let args = args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Must supply day as an argument.");
        return;
    }
    let day = i32::from_str(&args[1]);
    if day.is_err() {
        println!("Invalid day supplied.");
        return;
    }
    let day = day.unwrap();

    let file = "inputs/".to_owned() + if args.len() == 3 && args[2] == "test" {
        format!("day{}_test.txt", day)
    } else {
        if args.len() == 3 && args[2] == "massive" {
            format!("day{}_massive.txt", day)
        } else {
            format!("day{}_input.txt", day)
        }
    }.as_str();

    if let Ok(input) = read_to_string(file) {
        match day {
            1 => day1(input),
            2 => day2(input),
            3 => day3(input),
            4 => day4(input),
            5 => day5(input),
            6 => day6(&input),
            7 => day7(input),
            8 => day8(&input),
            9 => day9(&input),
            10 => day10(&input),
            11 => day11(&input),
            _ => println!("Day {} not complete.", day)
        }
    } else {
        println!("No input for day {} found.",day);
    }
}



