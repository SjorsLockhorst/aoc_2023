use std::{fs, i32};

fn main() {
    let contents = fs::read_to_string("../inputs/day2.txt").expect("couldn't open file'");
    // println!("{contents:?}");
    let games: Vec<String> = contents
        .lines()
        .map(|line| {
            line.to_string()
                .split_once(":")
                .unwrap()
                .1
                .replace(";", ",")
        })
        .collect();

    let scores: Vec<Vec<(i32, &str)>> = games
        .iter()
        .map(|game| {
            game.split(",")
                .map(|score| {
                    let cleaned = score.strip_prefix(" ").unwrap();
                    let number_color = cleaned.split_once(" ").unwrap();
                    let number = number_color.0.parse::<i32>().unwrap();
                    (number, number_color.1)
                })
                .collect()
        })
        .collect();

    let mut sum = 0;

    for (i, game_scores) in scores.iter().enumerate() {
        let valid_games: Vec<&i32> = game_scores
            .iter()
            .filter_map(|(score, color)| match color.to_owned() {
                "red" => {
                    if *score > 12 {
                        return None;
                    }
                    Some(score)
                }
                "green" => {
                    if *score > 13 {
                        return None;
                    }
                    Some(score)
                }
                "blue" => {
                    if *score > 14 {
                        return None;
                    }
                    Some(score)
                }
                _ => None,
            })
            .collect();

        if valid_games.len() == game_scores.len() {
            sum += i + 1
        }
    }

    println!("{sum:?}");

}
