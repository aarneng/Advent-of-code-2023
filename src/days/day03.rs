use crate::{Solution, SolutionPair};
use std::cmp;
use std::collections::HashMap;
///////////////////////////////////////////////////////////////////////////////

fn get_bounds(col: usize, row_low: usize, row_high: usize) -> Vec<(usize, usize)> {
    let min_l = 0;
    let max_l = 139;
    let lowest_col = cmp::max(col as i32 - 1, min_l) as usize;
    let highest_col = cmp::min(col + 1, max_l);
    let lowest_row = cmp::max(row_low as i32 - 1, min_l) as usize;
    let highest_row = cmp::min(row_high + 1, max_l);

    let mut ret_vec: Vec<(usize, usize)> = Vec::new();

    for c in lowest_col..=highest_col {
        for r in lowest_row..=highest_row {
            ret_vec.push((c, r));
        }
    }

    ret_vec
}


pub fn solve(input: std::str::Lines<'_>) -> SolutionPair {

    let lines: Vec<&str> = input.clone().collect();

    let mut total_sum = 0;

    let mut map: HashMap<Vec<(usize, usize)>, u32> = HashMap::new();

    for (idx_y, line) in lines.iter().enumerate() {

        let mut in_digit_block = false;
        let mut curr_start = 0;
        let mut curr_sum = 0;

        for (idx_x, character) in line.chars().enumerate() {
            if character.is_digit(10) && !in_digit_block {
                in_digit_block = true;
                curr_start = idx_x;
            }
            if (!character.is_digit(10) && in_digit_block) || (character.is_digit(10) && idx_x == 139) {
                if character.is_digit(10) && idx_x == 139 {
                    curr_sum = curr_sum * 10 + character.to_digit(10).unwrap();
                }
                let bounds = get_bounds(idx_y, curr_start, idx_x - 1);
                map.insert(bounds.clone(), curr_sum);
                in_digit_block = false;
                
                let mut found_char = false;
                
                for (y, x) in bounds {
                    let inp = lines[y].chars().nth(x).unwrap();
                    if !(inp.is_digit(10) || inp == '.') {
                        found_char = true;
                        break;
                    }
                }
                if found_char {
                    total_sum += curr_sum;
                }
                curr_sum = 0;
            }
            if character.is_digit(10) && in_digit_block {
                curr_sum = curr_sum * 10 + character.to_digit(10).unwrap();
            }
        }
    }

    let mut total_sum_pt2 = 0;
    for (idx_y, line) in lines.iter().enumerate() {
        for (idx_x, character) in line.chars().enumerate() {
            let ch = lines[idx_y].chars().nth(idx_x).unwrap();
            if ch == '*' {
                let mut prod = 1;
                let mut count = 0;
                for (key, value) in &map {
                    if key.contains(&(idx_y, idx_x)) {
                        count += 1;
                        prod *= value;
                    }
                }
                if count == 2 {
                    total_sum_pt2 += prod;
                }
            }
        }
    }

    (Solution::from(total_sum), Solution::from(total_sum_pt2))
}
