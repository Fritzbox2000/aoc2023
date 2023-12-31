#![allow(unused_imports)]
#![allow(dead_code)]
use aoc_runner_derive::{aoc, aoc_generator};

pub struct Card {
    winning_numbers: [u8; 10],
    actuall_numbers: [u8; 25],
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Card> {
    let mut output: Vec<Card> = Vec::new();
    input.lines().for_each(|line| {
        let (_, numbers) = line.split_at(9);
        let (winning_string, actuall_string) = numbers.split_at(32);
        let mut winning_numbers: [u8; 10] = [0; 10];
        let mut actuall_numbers: [u8; 25] = [0; 25];
        let mut counter = 0;
        for i in winning_string.split(' ') {
            match i {
                "" => {}
                "|" => {}
                other => {
                    winning_numbers[counter] = other.parse::<u8>().unwrap();
                    counter += 1;
                }
            }
        }
        counter = 0;

        for i in actuall_string.split(' ') {
            match i {
                "" => {}
                other => {
                    actuall_numbers[counter] = other.parse::<u8>().unwrap();
                    counter += 1;
                }
            }
        }
        output.push(Card {
            winning_numbers,
            actuall_numbers,
        })
    });
    output
}

#[aoc(day4, part1, main)]
pub fn main(cards: &[Card]) -> u32 {
    let mut output = 0;
    for card in cards {
        let mut card_score = 0;
        for actual_number in card.actuall_numbers {
            for winning_number in card.winning_numbers {
                if actual_number == winning_number {
                    card_score += 1;
                }
            }
        }
        if card_score != 0 {
            card_score = 1_u32 << (card_score - 1);
        }
        output += card_score;
    }
    output
}
#[aoc(day4, part1, iterator_i_hardly_know_her)]
pub fn main_iter(cards: &[Card]) -> u32 {
    let mut output = 0;
    for card in cards {
        let card_score = card
            .winning_numbers
            .iter()
            .filter(|&elem1| card.actuall_numbers.iter().any(|&elem2| elem1 == &elem2))
            .count();
        output += card_score.checked_sub(1).map_or(0, |out| 1_u32 << (out));
    }
    output
}

#[aoc(day4, part2, main)]
pub fn part2_main(cards: &[Card]) -> u32 {
    let mut output = 0;
    let mut wins: Vec<u32> = Vec::with_capacity(200);
    cards.iter().for_each(|card| {
        let mut card_score = 0;
        card.actuall_numbers.iter().for_each(|act| {
            card.winning_numbers.iter().for_each(|win| {
                if win == act {
                    card_score += 1;
                }
            });
            wins.push(card_score);
        })
    });
    let mut search_stack: Vec<usize> = (0..wins.len() - 1).collect();
    while let Some(top) = search_stack.pop() {
        output += 1;
        for to_add in 1..wins[top] {
            if top + (to_add as usize) + 1 < wins.len() {
                search_stack.push(top + (to_add as usize))
            }
        }
    }
    output
}

mod tests {
    use super::*;

    const TEST_INP: &str = "Card   1: 57 76 72 11  8 28 15 38 54 46 | 77 87 71 98 40  7 84 43 61 64  5 50 19 83 79 99 36 47  4 95 30 44 37 55 26
Card   2: 44 69 14 83 54 48 21  6 20 26 | 80 26 86  3  9  4 62 34 15 87 60 88 90 29 65 46 92 73 24 12 40 10 99 37 74
Card   3: 15 60 63 84 20 93 36 39 17 19 | 68 80 17 91 20 84 69 72 15 39  5 61 74 99 60 85 19 45 24 79 53 36  7 63 93
Card   4: 22 74 83 58 88 46  7 52 84  5 | 75 20 95  8 37 56 31 42 73 43 40 48  4 28 99 45 90 63 81 93 68 50 46 30  7
Card   5:  4 97 41 50 32 26 68 84  5 11 | 91 70 87  4 88 13 48 51 32 34 38 82 86 11  1 50 40 43 28  5 61 89 84 41 37";

    #[test]
    fn test_gen() {
        generator(TEST_INP);
        assert_eq!(1, 2);
    }
}
