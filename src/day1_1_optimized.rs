fn seek_first_digit<F>(input: &Vec<char>, idx: usize, walk: F) -> Option<u32>
where
    F: Fn(usize) -> usize,
{
    if idx >= input.len() {
        None
    } else {
        match input[idx].to_digit(10) {
            Some(digit) => Some(digit),
            None => seek_first_digit(input, walk(idx), walk),
        }
    }
}

pub fn day1_1_optimized(input: &Vec<String>) -> u32 {
    input.iter().fold(0, |acc, s| {
        let chars = s.chars().collect::<Vec<char>>();
        let first = seek_first_digit(&chars, 0, |x| x + 1).unwrap();
        let last = seek_first_digit(&chars, chars.len() - 1, |x| x - 1).unwrap();
        acc + (first * 10 + last)
    })
}
