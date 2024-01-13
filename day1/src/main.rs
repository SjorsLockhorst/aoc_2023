extern crate regex;

use regex::{Captures, Regex};
use std::fs;

fn part1() {
    let contents = fs::read_to_string("../inputs/day1.txt").expect("Couldn't open file'");
    let all_numbers = contents.lines().map(|line| {
        line.chars()
            .map(|c| match c {
                '0'..='9' => c.to_string(),
                _ => "".to_string(),
            })
            .collect::<Vec<String>>()
            .join("")
    });

    let nums: Vec<i32> = all_numbers
        .map(|num_str| {
            num_str.chars().nth(0).unwrap().to_string()
                + &num_str.chars().nth(num_str.len() - 1).unwrap().to_string()
        })
        .map(|num_str| num_str.parse().unwrap())
        .collect();

    let answer: i32 = nums.iter().sum();
    println!("Part 1:");
    println!("{answer:?}");
}

fn part2() {
    let contents = fs::read_to_string("../inputs/day1.txt").expect("Couldn't open file'");
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let changed_numbers: Vec<String> = contents
        .lines()
        .map(|line| {
            re.replace_all(line, |caps: &Captures| {
                match caps.get(1).unwrap().as_str() {
                    "one" => "1",
                    "two" => "2",
                    "three" => "3",
                    "four" => "4",
                    "five" => "5",
                    "six" => "6",
                    "seven" => "7",
                    "eight" => "8",
                    "nine" => "9",
                    _ => panic!("WHAA"), // This case should not happen due to the regex
                }
            })
            .to_string()
        })
    .collect();


    let all_numbers: Vec<String> = changed_numbers.iter().map(|line| {
        line.chars()
            .map(|c| match c {
                '0'..='9' => c.to_string(),
                _ => "".to_string(),
            })
            .collect::<Vec<String>>()
            .join("")
    }).collect();
    println!("{all_numbers:?}");

    let nums: Vec<u32> = all_numbers
        .iter()
        .map(|num_str| {
            num_str.chars().nth(0).unwrap().to_string()
                + &num_str.chars().nth(num_str.len() - 1).unwrap().to_string()
        })
        .map(|num_str| num_str.parse().unwrap())
        .collect();
    println!("{nums:?}");
    let answer: u32 = nums.iter().sum();
    // println!("{nums:?}");
    println!("Part 2:");
    println!("{answer:?}");
}
fn main() {
    // part1();
    part2();
}
