use std::{
    collections::{hash_map::RandomState, HashMap},
    hash::BuildHasher,
};

const DIGITS: &'static [&'static str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn starts_with(slice: &[char], target: &'static str) -> bool {
    if slice.len() < target.len() {
        false
    } else {
        target.chars().enumerate().all(|(i, c)| slice[i] == c)
    }
}

fn seek_first_digit<F, T>(
    digits_map: &HashMap<Vec<char>, u32, T>,
    line: &Vec<char>,
    walk: &F,
    idx: usize,
) -> Option<u32>
where
    F: Fn(usize) -> usize,
    T: BuildHasher,
{
    match line[idx] {
        char if (char as u32) - 0x30 < 10 => Some(char as u32 - 0x30),
        _ => {
            let slice = &line[idx..line.len()];
            DIGITS
                .iter()
                .find_map(|d| {
                    if starts_with(slice, d) {
                        digits_map.get(&d.chars().collect::<Vec<_>>()).copied()
                    } else {
                        None
                    }
                })
                .or_else(|| seek_first_digit(digits_map, line, walk, walk(idx)))
        }
    }
}

pub fn day1_2(input: &Vec<String>) -> u32 {
    let digits_map: HashMap<Vec<char>, u32, RandomState> = HashMap::from_iter(
        DIGITS
            .iter()
            .enumerate()
            .map(|(i, s)| (s.chars().collect::<Vec<char>>(), i as u32 + 1)),
    );
    input.iter().fold(0, |acc, s| {
        let chars = s.chars().collect::<Vec<char>>();
        let first = seek_first_digit(&digits_map, &chars, &|x| x + 1, 0).unwrap();
        let last = seek_first_digit(&digits_map, &chars, &|x| x - 1, chars.len() - 1).unwrap();
        acc + first * 10 + last
    })
}
