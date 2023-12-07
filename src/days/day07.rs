use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::cmp::Ordering;
///////////////////////////////////////////////////////////////////////////////

fn map_chars_to_count(input_str: &str) -> HashMap<char, u32> {
    let mut char_counts: HashMap<char, u32> = HashMap::new();

    for ch in input_str.chars() {
        *char_counts.entry(ch).or_insert(0) += 1;
    }

    char_counts
}

fn get_top_values(char_counts: &HashMap<char, u32>) -> Vec<(char, u32)> {
    let mut sorted_counts: Vec<_> = char_counts.iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(a.1));

    sorted_counts
        .iter()
        .map(|(&ch, &count)| (ch, count as u32))
        .collect()
}

fn compare_hands(hand1: &str, hand2: &str, game1: bool) -> Ordering {
    let mut strengths: Vec<char> = vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

    let mut count1 = map_chars_to_count(hand1);
    let mut count2 = map_chars_to_count(hand2);
    

    let mut top1 = get_top_values(&count1);
    let mut top2 = get_top_values(&count2);

    if !game1 {
        strengths = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

        if hand1 != "JJJJJ" {
            let mut topcount1 = top1[0].0;
            if topcount1 == 'J' {
                topcount1 = top1[1].0;
            }
            
            if let Some(j_value) = count1.remove(&'J') {
                count1
                .entry(topcount1)
                .and_modify(|e| { *e += j_value });
            }
            top1 = get_top_values(&count1);
        }
        
        if hand2 != "JJJJJ" {
            let mut topcount2 = top2[0].0;
            if topcount2 == 'J' {
                topcount2 = top2[1].0;
            }

            if let Some(j_value) = count2.remove(&'J') {
                count2
                    .entry(topcount2)
                    .and_modify(|e| { *e += j_value });
            }

            top2 = get_top_values(&count2);
        }
    }

    if top1[0].1 > top2[0].1 {
        return Ordering::Greater
    }
    if top1[0].1 < top2[0].1 {
        return Ordering::Less
    }
    if top1.len() > 1 && top1[1].1 > top2[1].1 {
        return Ordering::Greater
    }
    if top1.len() > 1 && top1[1].1 < top2[1].1 {
        return Ordering::Less
    }

    let indices1: Vec<usize> = hand1
        .chars()
        .map(|i| strengths.iter().position(|&r| r == i).unwrap())
        .collect();
    let indices2: Vec<usize> = hand2
        .chars()
        .map(|i| strengths.iter().position(|&r| r == i).unwrap())
        .collect();
    for (elem1, elem2) in indices1.iter().zip(indices2.iter()) {
        if elem1 < elem2 {
            return Ordering::Greater
        }
        if elem1 > elem2 {
            return Ordering::Less
        }
    }
    return Ordering::Equal;
}

pub fn solve(input: std::str::Lines<'_>) -> SolutionPair {

    let mut mapped: Vec<(&str, u64)> = input
        .map(|i| {
            let mut elems = i.split(" ");
            return (elems.next().unwrap(), elems.next().unwrap().parse().unwrap())
        })
        .collect();

    mapped.sort_by(|a, b| compare_hands(a.0, b.0, true));
    
    let mut sol1: u64 = 0;
    for (index, (a, b)) in mapped.iter().enumerate() {
        sol1 += (index as u64 + 1) * b;
    }
    
    mapped.sort_by(|a, b| compare_hands(a.0, b.0, false));

    let mut sol2: u64 = 0;
    for (index, (a, b)) in mapped.iter().enumerate() {
        sol2 += (index as u64 + 1) * b;
    }

    (Solution::from(sol1), Solution::from(sol2))
}
