use crate::{Solution, SolutionPair};
///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: std::str::Lines<'_>) -> SolutionPair {
    /*
    
    I wrote 2 solutions because i made a dumb typo in my first one 
    and only noticed it after I re-wrote the whole thing.

    the current code is the original (now fixed) solution

    Here is my "second" solution
    
    use std::collections::HashMap;
    
    fn get_first_last_digit(str: &str, part2: bool) -> String {    
        let mut all_str_nums = HashMap::from([
            ("0", "0"), ("1", "1"), ("2", "2"), ("3", "3"), ("4", "4"),
            ("5", "5"), ("6", "6"), ("7", "7"), ("8", "8"), ("9", "9"),
        ]);
        if part2 {
            let extension = HashMap::from([
                ("zero", "0"), ("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"),
                ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9"),
            ]);
            all_str_nums.extend(extension);
        }

        let mut min_idx = str.len() as i64;
        let mut max_idx: i64 = -1;

        let mut min_val: &str = "";
        let mut max_val: &str = "";
        for (num_str, num_val) in &all_str_nums {
            let idxs = str.match_indices(num_str).map(|(i, _)| i as i64);

            let low_idx = idxs.clone().next().unwrap_or(str.len() as i64);
            let high_idx = idxs.clone().last().unwrap_or(-1);
            
            if low_idx < min_idx {
                min_idx = low_idx;
                min_val = num_val;
            }
            if high_idx > max_idx {
                max_idx = high_idx;
                max_val = num_val;
            }
        }
        return min_val.to_owned() + max_val;
    }

    let filtered_input_pt1: Vec<u64> = input.clone()
        .map(|i| get_first_last_digit(i, false))
        .map(|i| i.parse().unwrap())
        .collect();

    let sol1: u64 = filtered_input_pt1.iter().sum();

    let filtered_input_pt2: Vec<u64> = input.clone()
        .map(|i| get_first_last_digit(i, true))
        .map(|i| i.parse().unwrap())
        .collect();

    // println!("{filtered_input_pt2:?}");
    let sol2: u64 = filtered_input_pt2.iter().sum();
    */
    
    let filtered_input_pt1: Vec<u64> = input.clone()
        .map(|i| i
            .chars()
            .filter(|chr| chr.is_digit(10))
            .collect())
        .map(|i: String| i.chars().next().unwrap().to_string() + &i.chars().last().unwrap().to_string())
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let sol1: u64 = filtered_input_pt1.iter().sum();

    let filtered_input_pt2: Vec<u64> = input.clone()
        .map(|i| i
            .replace("zero",  "z0o")
            .replace("one",   "o1e")
            .replace("two",   "t2o")
            .replace("three", "t3e")
            .replace("four",  "f4r")
            .replace("five",  "f5e")
            .replace("six",   "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine",  "n9e")
            .chars()
            .filter(|chr| chr.is_digit(10))
            .collect())
        .map(|i: String| i.chars().take(1).chain(i.chars().rev().take(1)).collect::<String>())
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let sol2: u64 = filtered_input_pt2.iter().sum();

    (Solution::from(sol1), Solution::from(sol2))
}
