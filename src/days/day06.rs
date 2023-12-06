use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
fn amt_ways(a: f64, b: f64) -> u64 {
    let lower_bound: u64 = ((a - (a*a - 4.0*b).sqrt()) / 2.0).floor()      as u64;
    let upper_bound: u64 = ((a + (a*a - 4.0*b).sqrt()) / 2.0 - 1.0).ceil() as u64;
    return upper_bound - lower_bound
}

pub fn solve(input: std::str::Lines<'_>) -> SolutionPair {

    let mut inp = input.clone();

    let elements1: Vec<f64> = inp
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s
            .parse()
            .unwrap())
        .collect();
    let elements2: Vec<f64> = inp
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s
            .parse()
            .unwrap())
        .collect();

    let pairs: Vec<(f64, f64)> = elements1.clone().into_iter().zip(elements2.clone().into_iter()).collect();

    let mut product1: u64 = 1;

    for (a, b) in pairs {
        product1 *= amt_ways(a, b);
    }

    let sol2: u64 = 0;

    // ugly parsing here lol
    let result1: f64 = elements1
        .clone()
        .iter()
        .map(|&num| num.to_string())
        .collect::<String>()
        .parse::<f64>()
        .unwrap() as f64;
    let result2: f64 = elements2
        .clone()
        .iter()
        .map(|&num| num.to_string())
        .collect::<String>()
        .parse::<f64>()
        .unwrap() as f64;

    let ans2 = amt_ways(result1, result2);

    (Solution::from(product1), Solution::from(ans2))
}
