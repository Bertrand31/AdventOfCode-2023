pub fn day1_1(input: &Vec<String>) -> u32 {
    input.iter().fold(0, |acc, s| {
        let digits = s.chars().flat_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
        let (first, _) = digits.split_first().unwrap();
        let (last, _) = digits.split_last().unwrap();
        acc + (first * 10 + last)
    })
}
