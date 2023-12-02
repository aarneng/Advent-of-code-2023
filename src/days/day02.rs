use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::cmp::max;
///////////////////////////////////////////////////////////////////////////////

fn parse_game_data(game_data: &str) -> Vec<Vec<(i32, String)>> {
    game_data
        .split("; ")
        .map(|sub_game| {
            sub_game
                .split(", ")
                .map(|s| {
                    let parts: Vec<_> = s.split_whitespace().collect();
                    let number = parts[0].parse::<i32>().unwrap();
                    let color = parts[1].to_string();
                    (number, color)
                })
                .collect()
        })
        .collect()
}

pub fn solve(input: std::str::Lines<'_>) -> SolutionPair {
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    let mut impossible_game_ids_sum: u64 = 0;
    let mut sum_powersets: u64 = 0;
    
    for game in input {
        let mut parts = game.split(": ");
        let id = parts.next().unwrap().split_whitespace().nth(1).unwrap().parse::<u64>().unwrap();
        let data = parts.next().unwrap();
        
        let game_amts_and_colors = parse_game_data(data);
        let mut sum_mult = 1;

        let mut max_color_amounts: HashMap<String, i32> = HashMap::from([
            ("red".to_string(), 0),
            ("green".to_string(), 0),
            ("blue".to_string(), 0),
        ]);

        for sub_game in &game_amts_and_colors {
            let mut curr_color_amounts: HashMap<String, i32> = HashMap::from([
                ("red".to_string(), 0),
                ("green".to_string(), 0),
                ("blue".to_string(), 0),
            ]);
            for (amt, color) in sub_game {
                let count_curr = curr_color_amounts.get_mut(color).unwrap();
                *count_curr += amt;
            }
            let red_bool: bool   = curr_color_amounts.get("red").unwrap()   > &12;
            let green_bool: bool = curr_color_amounts.get("green").unwrap() > &13;
            let blue_bool: bool  = curr_color_amounts.get("blue").unwrap()  > &14;
            if red_bool || green_bool || blue_bool {
                impossible_game_ids_sum += id * sum_mult;
                sum_mult = 0;
            }
            for (amt, color) in sub_game {
                let count_curr = curr_color_amounts.get_mut(color).unwrap();
                let count_max  = max_color_amounts.get_mut(color).unwrap();
                *count_max = max(*count_max, *count_curr);
            }
        }
        let mut power_of_cubeset = 1;
        for (key, value) in &max_color_amounts {
            power_of_cubeset *= value;
        }
        sum_powersets += power_of_cubeset as u64;
    }

    let sum_all_ids = 101 * 50;

    // println!("{split_input:?}");
    (Solution::from(sum_all_ids - impossible_game_ids_sum), Solution::from(sum_powersets))
}
