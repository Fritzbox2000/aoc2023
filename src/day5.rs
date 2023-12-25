#![allow(unused_imports)]
#![allow(dead_code)]
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug)]
pub struct SearchMap {
    functions: Vec<MappingFunction>,
}

#[derive(Debug)]
pub struct MappingFunction {
    output_start: u32,
    input_start: u32,
    length: u32,
}

#[aoc_generator(day5)]
pub fn main_generator(input: &str) -> (Vec<SearchMap>, Vec<u32>) {
    // let's get the seeds firstly:
    let seeds: Vec<u32> = input
        .lines()
        .nth(0)
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|i| i.parse::<u32>().unwrap())
        .collect();
    let mut maps: Vec<SearchMap> = Vec::with_capacity(7);
    let re = Regex::new(r"\n\s*\n").unwrap();
    re.split(input).skip(1).for_each(|i| {
        let mut functions: Vec<MappingFunction> = i
            .lines()
            .skip(1)
            .map(|j| {
                let numbers: Vec<u32> = j.split(' ').map(|k| k.parse::<u32>().unwrap()).collect();
                MappingFunction {
                    output_start: numbers[0],
                    input_start: numbers[1],
                    length: numbers[2],
                }
            })
            .collect();
        functions.sort_by(|a, b| a.input_start.cmp(&b.input_start));
        maps.push(SearchMap { functions })
    });
    (maps, seeds)
}

#[aoc(day5, part1, simple)]
pub fn main((input, seeds): &(Vec<SearchMap>, Vec<u32>)) -> u32 {
    let min = seeds
        .iter()
        .map(|i| {
            let mut start_value = i.clone();
            for map in input {
                // I would like to binary search this to make it quicker but I don't think that'll
                // do much
                for func in &map.functions {
                    if start_value >= func.input_start
                        && start_value <= (func.input_start + func.length)
                    {
                        start_value = dbg!(
                            ((start_value as i32) + (func.output_start as i32)
                                - (func.input_start as i32)) as u32
                        );
                        break;
                    }
                }
            }
            start_value
        })
        .min()
        .unwrap();
    return min;
}

mod tests {
    use super::*;
    const TEST_INP: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    #[test]
    fn test_main() {
        assert_eq!(main(&main_generator(TEST_INP)), 35)
    }
}
