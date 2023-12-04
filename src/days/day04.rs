use crate::{Solution, SolutionPair};
use std::collections::HashMap;
///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: std::str::Lines<'_>) -> SolutionPair {

    let lines = input
        .map(|i| i
            .split(": ")
            .nth(1)
            .unwrap())
        .map(|i| i
            .split('|')
            .map(|part| {
                part.trim()
                    .split_whitespace()
                    .map(|num_str| num_str.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>());
    
    let mut sum1: u64 = 0;
    let mut sum2 = 0;
    let mut multiplier_map: HashMap<usize, u64> = HashMap::new();

    for key in 0..=210 {
        multiplier_map.insert(key, 1);
    }

    for (idx, line) in lines.enumerate() {
        let winnings = &line[0];
        let numbers = &line[1];
        let mut count: u32 = 0;
        let mult: u64 = multiplier_map[&idx];
        for number in numbers {
            if winnings.contains(&number) {
                count += 1;
                *multiplier_map.entry(idx + count as usize).or_insert(0) += mult;
            }
        }
        sum1 += 2_u64.pow(count - 1);
        sum2 += mult;
    }

    (Solution::from(sum1), Solution::from(sum2))
}
