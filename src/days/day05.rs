use crate::{Solution, SolutionPair};
use regex::Regex;
use std::cmp::min;
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct AlmanacEntry {
    destination: u64,
    source: u64,
    delta: u64
}

fn extract_integers(inp: &str) -> Vec<u64> {
    Regex::new(r"\d+")
        .unwrap()
        .find_iter(inp)
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .collect()
}

fn map_entry(entry: u64, mappings: Vec<AlmanacEntry>) -> u64 {
    for map in mappings {
        let source = map.source;
        let delta = map.delta;
        if !(source <= entry && entry < source + delta) {
            continue;
        }
        let inp_diff = entry - source;
        let dest = map.destination;
        return dest + inp_diff;
    }
    return entry;
}

fn get_min_next_src(entry: u64, mappings: Vec<AlmanacEntry>) -> u64 {
    let mut min_next_src = u64::MAX;
    for map in mappings {
        let source = map.source;
        if source <= entry {
            continue;
        }
        min_next_src = min(min_next_src, source);
    }
    return min_next_src;
}

fn map_entry_range(entry_min: u64, entry_max: u64, mappings: Vec<AlmanacEntry>) -> Vec<(u64, u64)> {
    let mut all_mapped_entries: Vec<(u64, u64)> = Vec::new();
    let mut curr_entry = entry_min;

    while curr_entry < entry_max {
        let mapped = map_entry(curr_entry, mappings.clone());
        let next_source = get_min_next_src(curr_entry, mappings.clone());
        let delta = mapped - curr_entry;
        all_mapped_entries.push((curr_entry + delta, min(next_source - 1, entry_max) + delta));
        curr_entry = next_source;
    }

    return all_mapped_entries
}

pub fn solve(input: std::str::Lines<'_>) -> SolutionPair {

    let lines: Vec<&str> = input.clone().collect();

    let mut all_mappings: Vec<Vec<AlmanacEntry>> = Vec::with_capacity(7);

    for _ in 0..7 {
        all_mappings.push(Vec::new());
    }

    let seeds: Vec<u64> = extract_integers(lines[0]);

    let mut map_idx = 0;
    for i in &lines[3..] {
        if i == &"" {
            map_idx += 1;
            continue;
        }
        if i.contains("map") {
            continue;
        }
        let nums = extract_integers(i);
        let new_entry = AlmanacEntry {
            destination: nums[0],
            source: nums[1],
            delta: nums[2]
        };
        all_mappings[map_idx].push(new_entry);
    }

    
    let mut min_all_entries: u64 = u64::MAX;
    for seed in seeds.clone() {
        let mut temp_seed = seed;
        for i in 0..7 {
            temp_seed = map_entry(temp_seed, all_mappings[i].clone());
        }
        if temp_seed < min_all_entries {
            min_all_entries = temp_seed;
        }
    }
    
    let mut seed_ranges: Vec<(u64, u64)> = seeds
        .clone()
        .chunks(2)
        .map(|chunk| {
            let e1 = chunk[0];
            let e2 = chunk[1];
            (e1, e1 + e2)
        })
        .collect();

    for i in 0..7 {
        let mappings = all_mappings[i].clone();
        let mut new_seed_ranges: Vec<(u64, u64)> = Vec::new();
        for range in seed_ranges {
            let new_ranges = map_entry_range(range.0, range.1, mappings.clone());
            new_seed_ranges.extend(new_ranges.clone());
        }
        seed_ranges = new_seed_ranges;
    }

    let mut min_all_ranges = u64::MAX;
    for i in seed_ranges {
        min_all_ranges = min(min_all_ranges, i.0);
    }

    (Solution::from(min_all_entries), Solution::from(min_all_ranges))
}
