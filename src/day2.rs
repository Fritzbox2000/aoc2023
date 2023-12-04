#![allow(unused_imports)]
#![allow(dead_code)]
use aoc_runner_derive::{aoc, aoc_generator};
#[derive(Debug)]
pub struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}
pub struct Round {
    red: u32,
    green: u32,
    blue: u32,
}
#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let end_number = line[5..].find(':').unwrap();
            let game_id = line[5..5 + end_number].parse::<u32>().unwrap();
            let rounds: Vec<Round> = line[5 + end_number..].split(';').map(parse_round).collect();
            Game {
                id: game_id,
                red: rounds.iter().map(|s| s.red).max().unwrap(),
                green: rounds.iter().map(|s| s.green).max().unwrap(),
                blue: rounds.iter().map(|s| s.blue).max().unwrap(),
            }
        })
        .collect()
}

pub fn parse_round(input: &str) -> Round {
    let mut output: Round = Round {
        red: 0,
        green: 0,
        blue: 0,
    };
    for colour in input.split(',') {
        let col: Vec<&str> = colour.split(' ').collect();
        let num = col[1].parse::<u32>().unwrap();
        match col[2] {
            "red" => output.red = num,
            "green" => output.green = num,
            "blue" => output.blue = num,
            _ => {}
        }
    }
    output
}
#[aoc[day2, part1, main]]
pub fn part_1_main(input: &[Game]) -> u32 {
    let mut output: u32 = 0;
    for game in input {
        if game.red <= 12 && game.green <= 13 && game.blue <= 14 {
            output += game.id;
        }
    }
    output
}

#[aoc[day2, part2, main]]
pub fn part_2_main(input: &[Game]) -> u32 {
    let mut output: u32 = 0;
    for game in input {
        output += game.red * game.blue * game.green;
    }
    output
}

mod tests {
    use super::*;

    const TEST_INP: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    #[test]
    fn test_generator() {
        println!("{:?}", generator(TEST_INP));
    }

    #[test]
    fn test_generator_internal() {
        parse_round(" 3 blue, 4 red");
    }

    #[test]
    fn test_main() {
        assert_eq!(8, part_1_main(&generator(TEST_INP)));
    }
}
