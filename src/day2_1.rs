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

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<GameSet>,
}

#[derive(Debug)]
struct GameMax {
    id: usize,
    max_values: GameSet,
}

fn parse_line(line: &String) -> Game {
    let mut split_line = line.split(":");
    let game_str = split_line.next().unwrap().to_owned();
    let id = game_str.split(" ").last().unwrap().parse::<usize>().unwrap();
    let sets = split_line
        .next()
        .unwrap()
        .split(";")
        .map(|s| s.trim())
        .map(|set_str| {
            let colours = set_str.split(",").map(|c| c.trim());
            let pairs = colours.map(|p| {
                let mut colour = p.split(" ");
                let value = colour.next().unwrap().parse::<usize>().unwrap();
                let name = colour.next().unwrap().to_owned();
                (name, value)
            });
            let hashmap: HashMap<String, usize, RandomState> = HashMap::from_iter(pairs);
            GameSet {
                blue: hashmap.get("blue").unwrap_or(&0_usize).to_owned(),
                red: hashmap.get("red").unwrap_or(&0_usize).to_owned(),
                green: hashmap.get("green").unwrap_or(&0_usize).to_owned(),
            }
        })
        .collect::<Vec<GameSet>>();
    Game { id, sets }
}

fn collapse_game(game: Game) -> GameMax {
    let max_set = game.sets.iter().fold(
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
        id: game.id,
        max_values: max_set,
    }
}

pub fn day2_1(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(parse_line)
        .map(collapse_game)
        .filter_map(|g_max| {
            if g_max.max_values.blue <= 14
                && g_max.max_values.green <= 13
                && g_max.max_values.red <= 12
            {
                Some(g_max.id)
            } else {
                None
            }
        })
        .sum()
}
