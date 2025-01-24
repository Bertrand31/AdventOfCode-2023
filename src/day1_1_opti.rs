// This function assumes the input vec contains at least one digit
fn seek_first_digit<F>(input: &Vec<char>, idx: usize, walk: F) -> u32
where
    F: Fn(usize) -> usize,
{
    match input[idx] as u32 - 0x30 {
        digit if digit < 10 => digit,
        _ => seek_first_digit(input, walk(idx), walk),
    }
}

pub fn day1_1_opti(input: &Vec<String>) -> u32 {
    let mut sum = 0;
    for s in input {
        let chars = s.chars().collect::<Vec<char>>();
        let first = seek_first_digit(&chars, 0, |x| x + 1);
        let last = seek_first_digit(&chars, chars.len() - 1, |x| x - 1);
        sum += first * 10 + last;
    }
    sum
}
