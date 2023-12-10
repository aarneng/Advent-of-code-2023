use crate::{Solution, SolutionPair};
use std::collections::HashMap;
///////////////////////////////////////////////////////////////////////////////

fn find_index(vec_str: Vec<&str>, target_char: char) -> (i64, i64) {
    for (index, &s) in vec_str.iter().enumerate() {
        if let Some(pos) = s.find(target_char) {
            return (pos as i64, index as i64);
        }
    }
    return (-1, -1)
}

fn get_nearby_indices(current: (i64, i64), data: Vec<&str>) -> Vec<(i64, i64)> {
    let mut ret_vec: Vec<(i64, i64)> = vec![];
    if current.0 > 0 {
        ret_vec.push((current.0 - 1, current.1));
    }
    if current.1 > 0 {
        ret_vec.push((current.0, current.1 - 1));
    }
    if current.0 < data[0].len() as i64 - 1 {
        ret_vec.push((current.0 + 1, current.1));
    }
    if current.1 < data.len() as i64 - 1 {
        ret_vec.push((current.0, current.1 + 1));
    }
    return ret_vec
}

fn get_char_at_index(data: Vec<&str>, idx: (i64, i64)) -> &str {
    let idx_x = idx.0 as usize;
    let idx_y = idx.1 as usize;
    return &data[idx_y][idx_x..idx_x + 1]
}

fn char_to_delta_idx(c: &str) -> Vec<(i64 , i64)> {
    return match c {
        "|" => vec![(0,  1),  ( 0, -1)],
        "-" => vec![(1,  0),  (-1,  0)],
        "L" => vec![(0, -1),  ( 1,  0)],
        "J" => vec![(0, -1),  (-1,  0)],
        "7" => vec![(-1, 0),  ( 0,  1)],
        "F" => vec![(1,  0),  ( 0,  1)],
        _ =>   vec![(0,  0)]
    };
}

fn get_deltas_connect_start(data: Vec<&str>) -> Vec<(i64, i64)> {
    let start_idx = find_index(data.clone(), 'S');
    let mut ret_vec: Vec<(i64, i64)> = vec![];
    for idx in get_nearby_indices(start_idx, data.clone()) {
        let c = get_char_at_index(data.clone(), idx);
        let deltas = char_to_delta_idx(c);
        for delta in deltas {
            if idx.0 + delta.0 == start_idx.0 && idx.1 + delta.1 == start_idx.1 {
                ret_vec.push(delta);
                break;
            }
        }
    }
    return ret_vec
}

fn get_pipes_connect_start(data: Vec<&str>) -> Vec<(i64, i64)> {
    let start_idx = find_index(data.clone(), 'S');
    return get_deltas_connect_start(data)
        .iter()
        .map(|i| (start_idx.0 + i.0, start_idx.1 + i.1))
        .collect();
}

fn get_next(current: (i64, i64), previous: (i64, i64), data: Vec<&str>) -> (i64, i64) {
    let deltas = char_to_delta_idx(get_char_at_index(data, current));
    for delta in deltas {
        if !(current.0 + delta.0 == previous.0 && current.1 + delta.1 == previous.1) {
            return (current.0 + delta.0, current.1 + delta.1)
        }
    }
    return previous; // TODO: invent better error state?
}


pub fn solve(input: std::str::Lines<'_>) -> SolutionPair {

    let lines: Vec<&str> = input.collect();

    let start = find_index(lines.clone(), 'S');
    let edge_start = vec![
            vec![(0,   1),  ( 0, -1)],
            vec![(0,  -1),  ( 1,  0)],
            vec![(0,  -1),  (-1,  0)]
        ].contains(&get_deltas_connect_start(lines.clone()));

    let mut prev = start;
    let mut curr = get_pipes_connect_start(lines.clone())[0];
    let mut count = 0;
    let mut all_tiles: HashMap<(i64, i64), bool> = HashMap::from([(curr, true)]);
    while curr != start {
        let cpy = get_next(curr, prev, lines.clone());
        prev = curr;
        curr = cpy;
        count += 1;
        all_tiles.insert(curr, true);
    }

    let mut innercount: u64 = 0;
    for (y, line) in lines.iter().enumerate() {
        let cy = y as i64;
        let mut is_odd_pipe_char_count = false;
        for (x, c) in line.chars().enumerate() {
            let cx = x as i64;
            let contains = all_tiles.contains_key(&(cx, cy));
            if ("|JL".contains(c) || (c == 'S' && edge_start)) && contains {
                is_odd_pipe_char_count = !is_odd_pipe_char_count;
            }
            innercount += (is_odd_pipe_char_count && !contains) as u64;
        }
    }

    (Solution::from((count + 1) >> 1), Solution::from(innercount))
}
