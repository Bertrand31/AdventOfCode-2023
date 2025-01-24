use std::{
    cmp::max,
    collections::{hash_map::RandomState, HashMap},
};

#[derive(Debug)]
struct GameSet {
    blue: usize,
    red: usize,
    green: usize,
}

struct Game(Vec<GameSet>);

struct GameMax {
    max_values: GameSet,
}

fn parse_line(line: &String) -> Game {
    let sets = line
        .split(":")
        .last()
        .unwrap()
        .split(";")
        .map(|s| s.trim())
        .map(|set_str| {
            let colour_pairs = set_str.split(",").map(|c| c.trim()).map(|p| {
                let mut colour = p.split(" ");
                let value = colour.next().unwrap().parse::<usize>().unwrap();
                let name = colour.next().unwrap().to_owned();
                (name, value)
            });
            let hashmap: HashMap<String, usize, RandomState> = HashMap::from_iter(colour_pairs);
            GameSet {
                blue: hashmap.get("blue").unwrap_or(&0_usize).to_owned(),
                red: hashmap.get("red").unwrap_or(&0_usize).to_owned(),
                green: hashmap.get("green").unwrap_or(&0_usize).to_owned(),
            }
        })
        .collect::<Vec<GameSet>>();
    Game(sets)
}

fn collapse_game(game: Game) -> GameMax {
    let max_set = game.0.iter().fold(
        GameSet {
            blue: 0,
            red: 0,
            green: 0,
        },
        |acc, item| GameSet {
            blue: max(acc.blue, item.blue),
            red: max(acc.red, item.red),
            green: max(acc.green, item.green),
        },
    );
    GameMax {
        max_values: max_set,
    }
}

pub fn day2_2(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(parse_line)
        .map(collapse_game)
        .map(|g_max| g_max.max_values.blue * g_max.max_values.green * g_max.max_values.red)
        .sum()
}
