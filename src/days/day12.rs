use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

fn count_ways(n: usize, params: &[usize]) -> usize {
    let m = params.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for &param in params.iter() {
        for i in (param..=n).rev() {
            dp[i] += dp[i - param];
        }
    }

    dp[n]
}

pub fn solve(input: std::str::Lines<'_>) -> SolutionPair {
    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    let n = 6;
    let lengths = vec![2, 1];
    let result = count_ways(n, &lengths);
    println!("{result}");

    (Solution::from(sol1), Solution::from(sol2))
}
