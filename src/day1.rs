use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

fn conv_number(number: &str) -> Option<i32> {
    match number {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "zero" => Some(0),
        &_ => None,
    }
}
#[aoc[day1,part1,middle_out]]
pub fn part_1_main(input: &str) -> u32 {
    let mut output = 0;
    for line in input.lines().map(|line| line.as_bytes()) {
        let mut front_pointer: usize = (((line.len() - 1) as f32) / 2.0).floor() as usize;
        let mut back_pointer: usize = (((line.len() - 1) as f32) / 2.0).ceil() as usize;
        let mut front_val: Option<u8>;
        let mut back_val: Option<u8>;

        front_val = check_char(line[front_pointer]);
        back_val = check_char(line[back_pointer]);
        while front_pointer != 0 {
            front_pointer -= 1;
            back_pointer += 1;
            front_val = match (check_char(line[front_pointer])) {
                Some(value) => Some(value),
                None => front_val,
            };
            back_val = match (check_char(line[back_pointer])) {
                Some(value) => Some(value),
                None => back_val,
            };
        }
        if front_val == None {
            front_val = Some(check_forward(line));
        }
        if back_val == None {
            back_val = Some(check_back(line));
        }
        output += ((10 * front_val.unwrap()) + back_val.unwrap()) as u32;
    }
    output
}

#[aoc(day1, part1, outer_in)]
pub fn outer_in(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            outer_in_internal(bytes)
        })
        .sum()
}

pub fn outer_in_internal(line: &[u8]) -> u32 {
    let mut front_pointer = 0 as usize;
    let mut back_pointer = (line.len() - 1) as usize;
    let mut front_val: Option<u8> = None;
    let mut back_val: Option<u8> = None;
    while front_val == None || back_val == None {
        front_val = match check_char(line[front_pointer]) {
            Some(value) => Some(value),
            None => front_val,
        };
        back_val = match check_char(line[back_pointer]) {
            Some(value) => Some(value),
            None => back_val,
        };
        front_pointer += 1;
        back_pointer -= 1;
    }
    ((front_val.unwrap() * 10) + back_val.unwrap()) as u32
}

pub fn check_char(char: u8) -> Option<u8> {
    if char >= b'0' && char <= b'9' {
        return Some((char - b'0') as u8);
    }
    None
}

pub fn check_back(line: &[u8]) -> u8 {
    let mut pointer: usize = (((line.len() as f32) - 1.0) / 2.0).floor() as usize;
    match check_char(line[pointer]) {
        Some(value) => return value,
        None => {}
    };
    while pointer != 0 {
        pointer -= 1;
        match check_char(line[pointer]) {
            Some(value) => return value,
            None => {}
        };
    }
    0
}

pub fn check_forward(line: &[u8]) -> u8 {
    let mut pointer: usize = (((line.len() as f32) - 1.0) / 2.0).ceil() as usize;
    match check_char(line[pointer]) {
        Some(value) => return value,
        None => {}
    };
    while pointer != line.len() - 1 {
        pointer += 1;
        match check_char(line[pointer]) {
            Some(value) => return value,
            None => {}
        };
    }
    0
}

#[aoc(day1, part2, main)]
pub fn part_2_main(input: &str) -> u32 {
    let mut output = 0;
    for line in input.lines().map(|line| line.as_bytes()) {}
    0
}

mod tests {
    use super::{outer_in, part_1_main};
    const test_inp: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    #[test]
    fn test_simple_input() {
        assert_eq!(part_1_main(test_inp), 142)
    }

    #[test]
    fn test_simple_input2() {
        assert_eq!(outer_in(test_inp), 142)
    }
}
