use crate::{Solution, SolutionPair};
use std::cmp::min;
use std::cmp::max;
use std::iter::repeat;
///////////////////////////////////////////////////////////////////////////////

fn process_input(input: std::str::Lines<'_>) -> (Vec<String>, Vec<(u64, u64)>) {
    let mut result = Vec::new();
    let mut hash_indices: Vec<(u64, u64)> = Vec::new();

    let row_len = input.clone().next().unwrap().len();
    let mut rows_with_hash: Vec<bool> = vec![false; row_len];

    for col_index in 0..row_len {
        let row_has_hash = input.clone().any(|row| row.chars().nth(col_index).unwrap() == '#');
        rows_with_hash[col_index] = row_has_hash;
    }

    for (row_index, line) in input.clone().enumerate() {
        
        let col_has_hash = line.chars().any(|c| c == '#');
        if !col_has_hash {
            result.push(repeat('2').take(row_len).collect());
            continue;
        }
        let mut new_line_chars = Vec::with_capacity(row_len);

        for (col_index, ch) in line.chars().enumerate() {
            if ch == '.' {
                let row_has_hash: bool = rows_with_hash[col_index];
                new_line_chars.push(if row_has_hash { '1' } else { '2' });
            } else {
                new_line_chars.push(ch);
                hash_indices.push((row_index as u64, col_index as u64));
            }
        }

        result.push(new_line_chars.into_iter().collect());

    }

    (result, hash_indices)
}

fn generate_indices(x1: usize, y1: usize, x2: usize, y2: usize) -> Vec<(usize, usize)> {
    let mut indices = Vec::new();

    for x in min(x1, x2) + 1..=max(x1, x2) {
        indices.push((x, y1));
    }

    for y in min(y1, y2) + 1..=max(y1, y2) {
        indices.push((x2, y));
    }

    indices
}

pub fn solve(input: std::str::Lines<'_>) -> SolutionPair {

    let (res, idxs) = process_input(input);

    let mut count_pairs = 0;
    let mut allsum1: u64 = 0;
    let mut allsum2: u64 = 0;
    for (idx1, coords1) in idxs.iter().enumerate() {
        let x1 = coords1.1;
        let y1 = coords1.0;
        for (idx2, coords2) in idxs[idx1 + 1..].iter().enumerate() {
            let x2 = coords2.1;
            let y2 = coords2.0;
            count_pairs += 1;
            let mut sum1: u64 = 0;
            let mut sum2: u64 = 0;
            for index in generate_indices(x1 as usize, y1 as usize, x2 as usize, y2 as usize) {
                let x = index.1 as usize;
                let y = index.0;
                match res[x].chars().nth(y) {
                    Some('2') => {
                        sum1 += 2;
                        sum2 += 1000000;
                    },
                    _ => {
                        sum1 += 1;
                        sum2 += 1;
                    },
                }
            }
            allsum1 += sum1;
            allsum2 += sum2;
        }
    }

    (Solution::from(allsum1), Solution::from(allsum2))
}
