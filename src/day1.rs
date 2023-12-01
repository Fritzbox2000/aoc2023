use aoc_runner_derive::{aoc, aoc_generator};

#[aoc[day1,part1,gen]]
pub fn all_in_one(input: &str) -> u32 {
    let mut output = 0;
    for line in input.lines().map(|line| line.as_bytes()) {
        let mut front_pointer: usize = (((line.len() as f32) - 1.0) / 2.0).floor() as usize;
        let mut back_pointer: usize = (((line.len() as f32) - 1.0) / 2.0).ceil() as usize;
        let mut front_val: Option<u32> = None;
        let mut back_val: Option<u32> = None;

        front_val = match check_char(line[front_pointer]) {
            Some(value) => Some(value),
            None => front_val,
        };
        back_val = match check_char(line[back_pointer]) {
            Some(value) => Some(value),
            None => back_val,
        };
        while front_pointer != 0 {
            front_pointer -= 1;
            back_pointer += 1;
            front_val = match check_char(line[front_pointer]) {
                Some(value) => Some(value),
                None => front_val,
            };
            back_val = match check_char(line[back_pointer]) {
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
        output += (10 * front_val.unwrap()) + back_val.unwrap();
    }
    output
}
pub fn check_char(char: u8) -> Option<u32> {
    if char >= b'0' && char <= b'9' {
        return Some((char - b'0') as u32);
    }
    None
}

pub fn check_back(line: &[u8]) -> u32 {
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

pub fn check_forward(line: &[u8]) -> u32 {
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

#[test]
fn test_simple_input() {
    let test_inp = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    assert_eq!(all_in_one(test_inp), 142)
}
