use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

struct HeatMap(HashSet<Coordinate>);

fn make_heatmap(grid: &Vec<Vec<char>>) -> HeatMap {
    let set = grid
        .iter()
        .enumerate()
        .fold(HashSet::new(), |acc, (y, row)| {
            row.iter().enumerate().fold(acc, |mut set, (x, char)| {
                let char_code = *char as u8;
                if char_code < 48 && char_code != 46 {
                    set.insert(Coordinate { x, y });
                }
                set
            })
        });
    HeatMap(set)
}

#[derive(Debug)]
struct GridNumber {
    number: usize,
    coordinates: (Coordinate, Coordinate),
}

fn parse_row(row: Vec<char>, current_y: usize) -> Vec<GridNumber> {
    let number_start = row
        .iter()
        .enumerate()
        .skip_while(|(_, c)| (**c as u8) < 48 || (**c as u8) > 57)
        .collect::<Vec<_>>();
    if number_start.is_empty() {
        vec![]
    } else {
        let digits = number_start.iter().take_while(|(_, c)| (**c as usize) >= 48 && (**c as usize) <= 57);
        let result = parse_row(rest, current_y);
        // result.push(value)
        result
    }
}

fn parse_numbers(grid: &Vec<Vec<char>>) -> Vec<GridNumber> {
    grid.iter().enumerate().fold(Vec::new(), |acc, (y, row)| {
        acc.extend(parse_row(row, y));
        acc
    })
}

pub fn day3_1(input: &Vec<String>) -> usize {
    let grid = input
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let heatmap = make_heatmap(&grid);
    println!("{:?}", heatmap.0);
    3_usize
}
