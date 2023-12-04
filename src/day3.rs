use aoc_runner_derive::{aoc, aoc_generator};

pub struct Coord {
    x: usize,
    y: usize,
}

#[aoc[day3,part1, main]]
pub fn main(input: &str) -> u32 {
    // ok so dumb stuff is this one
    // Search for each number in the string and then check for characters outside of it
    // calculate line length
    let length = input.find('\n').unwrap();
    let height = input.lines().count();
    let mut output = 0;
    // ok maybe I'm over thinking stuff
    let mut index: Option<usize> = Some(0);
    while index != None {
        println!("INDEX: {:?}", index);
        // println!(
        //     "test: {:?}",
        //     input[100..]
        //         .bytes()
        //         .position(|c| c as u8 <= b'0' || c as u8 >= b'9')
        // );
        let first_index = input[index.unwrap()..]
            .bytes()
            .position(|c| c as u8 >= b'0' || c as u8 <= b'9');
        println!("first_index: {:?}", first_index);
        match first_index {
            Some(first_index_i) => {
                let second_index = input[first_index_i..]
                    .bytes()
                    .position(|c| c as u8 <= b'0' || c as u8 >= b'9')
                    .unwrap();
                println!("Second_index {:?}", second_index);
                let number = input[first_index_i..first_index_i + second_index]
                    .parse::<u32>()
                    .unwrap();
                println!("{:?}", number);
                // now I need to search around this number to find characters
                for Coord in make_rect(
                    conv_index_coord(first_index_i, length),
                    second_index - 1 - first_index_i,
                    length,
                    height,
                ) {
                    let char_at_coord = get_char_at_coord(input, Coord, length);
                    if char_at_coord < b'0' || char_at_coord > b'9' {
                        output += number
                    }
                }
                index = Some(index.unwrap() + first_index_i + second_index + 1);
            }
            None => index = None,
        }
        println!("{:?}", index);
    }
    output
}

pub fn get_char_at_coord(input: &str, pos: Coord, length: usize) -> u8 {
    input.as_bytes()[(pos.y * length) + pos.x]
}

pub fn conv_index_coord(index: usize, length: usize) -> Coord {
    Coord {
        x: index % length,
        y: index / length,
    }
}

pub fn make_rect(pos: Coord, length: usize, max_length: usize, max_height: usize) -> Vec<Coord> {
    let mut output: Vec<Coord> = Vec::new();
    // let's add the front ones
    if pos.x > 0 {
        if pos.y > 0 {
            output.push(Coord {
                x: pos.x - 1,
                y: pos.y - 1,
            });
        }
        output.push(Coord {
            x: pos.x - 1,
            y: pos.y,
        });
        if pos.y < max_height {
            output.push(Coord {
                x: pos.x - 1,
                y: pos.y + 1,
            });
        }
    }
    // now let's loop over each one and do the char above and below
    for i in 0..(length - 1) {
        if pos.y > 0 {
            output.push(Coord {
                x: pos.x + i,
                y: pos.y - 1,
            });
        }
        if pos.y < max_height {
            output.push(Coord {
                x: pos.x + i,
                y: pos.y + 1,
            });
        }
    }
    // finally let's do the back end
    if pos.x + length < max_length {
        if pos.y > 0 {
            output.push(Coord {
                x: pos.x + length,
                y: pos.y - 1,
            });
        }
        output.push(Coord {
            x: pos.x + length,
            y: pos.y,
        });
        if pos.y < max_height {
            output.push(Coord {
                x: pos.x + length,
                y: pos.y + 1,
            });
        }
    }
    output
}

mod tests {
    use super::*;

    const TEST_INP: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    #[test]
    fn test_main() {
        assert_eq!(main(TEST_INP), 4361);
    }
}
