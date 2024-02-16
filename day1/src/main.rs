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
    let changed_numbers: u32 = contents
        .lines()
        .map(|line| {
            line.to_string()
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum();


    // println!("{nums:?}");
    println!("Part 2:");
    println!("{answer:?}");
}
fn main() {
    part1();
    part2();
}
