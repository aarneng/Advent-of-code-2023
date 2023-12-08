use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use num::integer::lcm;

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: std::str::Lines<'_>) -> SolutionPair {

    let sol2: u64 = 0;

    let mut lines = input.clone();

    let instructions = lines.next().unwrap();
    lines.next();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();


    for line in lines {
        let parts: Vec<&str> = line.split(" = ").collect();
        let key = parts[0];
        let values = parts[1].trim_matches(|c| c == '(' || c == ')').split(',').collect::<Vec<&str>>();
        
        if values.len() == 2 {
            map.insert(key, (values[0].trim(), values[1].trim()));
        }
    }

    let mut count: u64 = 0;
    let mut curr_element = "AAA";
    while curr_element != "ZZZ" {
        let idx = count as usize % instructions.len();
        let instruction = instructions.as_bytes()[idx];
        if instruction == b'L' {
            curr_element = map[curr_element].0;
        }
        if instruction == b'R' {
            curr_element = map[curr_element].1;
        }
        count += 1;
    }

    let mut counts2: Vec<u64> = Vec::new();
    let all_curr_elements: Vec<&str> = map
        .keys().
        filter(|&&i| i.ends_with("A"))
        .cloned()
        .collect();

    for el in all_curr_elements {
        let mut curr_elem = el;
        let mut curr_count: u64 = 0;

        while !curr_elem.ends_with("Z") {
            let idx = curr_count as usize % instructions.len();
            let instruction = instructions.as_bytes()[idx];
            if instruction == b'L' {
                curr_elem = map[curr_elem].0;
            }
            if instruction == b'R' {
                curr_elem = map[curr_elem].1;
            }
            curr_count += 1;
        }
        counts2.push(curr_count);
    }

    let sol2 = counts2.into_iter().fold(1, |acc, x| lcm(acc, x));

    (Solution::from(count), Solution::from(sol2))
}
