use advent_of_code_2023::day1_1::day1_1;
use advent_of_code_2023::day1_1_opti::day1_1_opti;
use advent_of_code_2023::utils::read_lines;

use std::time::Instant;

fn benchmark<F, O>(fun: F) -> O
where
    F: Fn() -> O,
{
    let mut runtimes = Vec::new();
    for _ in 0..1000 {
        let now = Instant::now();
        {
            fun();
        }
        runtimes.push(now.elapsed().as_micros() as i128);
    }
    let sum_runtimes: i128 = runtimes.iter().sum();
    println!("Avg run: {:.2?}μs", sum_runtimes / 1000_i128);
    fun()
}

fn main() {
    let day1_1_input = read_lines("input-day1-1.txt").unwrap();
    println!("Day1_1: {}", benchmark(|| day1_1(&day1_1_input)));
    println!("Day1_1_opti: {}", benchmark(|| day1_1_opti(&day1_1_input)));
}
