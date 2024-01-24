use advent_of_code_2023::day1_1::day1_1;
use advent_of_code_2023::day1_1_optimized::day1_1_optimized;
use advent_of_code_2023::utils::read_lines;

fn main() {
    let day1_1_input = read_lines("input-day1-1.txt").unwrap();
    println!("Day1_1: {}", day1_1(&day1_1_input));
    println!("Day1_1_optimized: {}", day1_1_optimized(&day1_1_input));
}
