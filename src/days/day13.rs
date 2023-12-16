use crate::{Solution, SolutionPair};
use std::cmp::min;
use std::cmp::max;
use std::collections::HashMap;
///////////////////////////////////////////////////////////////////////////////

fn is_palindrome(s: &str) -> bool {
    let clean_str = s.chars();
    let reversed_str = clean_str.clone().rev();

    clean_str.eq(reversed_str)
}

fn get_all_indices(l: i64) -> Vec<(i64, i64)> {
    let mut ret: Vec<(i64, i64)> = vec![];
    let mut idx_low: i64 = -l + 2;
    let mut idx_high: i64 = 2;
    while idx_low < l {
        ret.push((max(idx_low, 0), min(idx_high, l)));
        idx_low += 2;
        idx_high += 2;
    }
    return ret;
}

fn process_mat(mat: Vec<&str>) -> u64 {
    let mut curr_working_indices: Vec<(i64, i64)> = get_all_indices(mat.len() as i64);

    for elem in &curr_working_indices {
        let mut low_idx = elem.0;
        let mut high_idx = elem.1 - 1;
        while low_idx < high_idx && mat[low_idx as usize] == mat[high_idx as usize] {
            low_idx += 1;
            high_idx -= 1;
        }
        if mat[low_idx as usize] == mat[high_idx as usize] {
            return  100 * low_idx as u64;
        }
    }

    curr_working_indices = get_all_indices(mat[0].len() as i64);
    for (i, s) in mat.clone().iter().enumerate() {
        let mut c_w_i_copy: Vec<(i64, i64)> = vec![];
        for elem in &curr_working_indices {
            if is_palindrome(&s[elem.0 as usize .. elem.1 as usize]) {
                c_w_i_copy.push(*elem);
            }
        }
        curr_working_indices = c_w_i_copy;
    }
    if curr_working_indices.len() >= 1 {
        let elem = curr_working_indices[0];
        return ((elem.0 + elem.1) >> 1) as u64;
    }


    return 0;
}

fn amt_deltas(s1: &str, s2: &str) -> u64 {
    let len = s1.len();
    let mut amt: u64 = 0;

    for i in 0..len {
        let char1 = s1.chars().nth(i).unwrap();
        let char2 = s2.chars().nth(i).unwrap();

        if char1 != char2 {
            amt += 1;
        }
    }
    return amt;
}

fn process_mat_2(mat: Vec<&str>) -> u64 {
    let curr_working_indices: Vec<(i64, i64)> = get_all_indices(mat.len() as i64);
    let len = mat[0].len();
    
    for elem in &curr_working_indices {
        let mut deltas = 0;
        let mut low_idx = elem.0;
        let mut high_idx = elem.1 - 1;

        while low_idx < high_idx {
            deltas += amt_deltas(mat[low_idx as usize], mat[high_idx as usize]);
            low_idx += 1;
            high_idx -= 1;
        }
        if deltas != 1 {
            continue;
        }
        return  100 * low_idx as u64;
    }

    let mut test: HashMap<(i64, i64), u64> = get_all_indices(mat[0].len() as i64)
        .into_iter()
        .map(|elem| (elem, 0))
        .collect();

    for (i, s) in mat.clone().iter().enumerate() {
        let mut c_w_i_copy: HashMap<(i64, i64), u64> = HashMap::new();
        for (elem, value) in &test {
            let s1 = &s[elem.0 as usize .. elem.1 as usize];
            let s2: String = s1.chars().rev().collect();
            let d = amt_deltas(s1, &s2) / 2;
            if value + d <= 1 {
                c_w_i_copy.insert(*elem, value + d);
            }
        }
    
        test = c_w_i_copy;
    }

    for (elem, value) in &test {
        if *value == 1 {
            return ((elem.0 + elem.1) >> 1) as u64;
        }
    }

    return 0;

}

pub fn solve(input: std::str::Lines<'_>) -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = 0;
    let mut sol2: u64 = 0;

    let mut mat: Vec<&str> = vec![];
    for (i, s) in input.clone().enumerate() {
        if s == "" {
            sol1 += process_mat(mat.clone());
            let a = process_mat_2(mat);
            sol2 += a;
            mat = vec![];
            continue;
        }
        mat.push(s);
    }

    sol1 += process_mat(mat.clone());
    let a = process_mat_2(mat);
    sol2 += a;

    (Solution::from(sol1), Solution::from(sol2))
}
