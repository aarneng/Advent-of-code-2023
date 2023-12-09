use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: std::str::Lines<'_>) -> SolutionPair {

    let mut sol1: u64 = 0;
    let mut sol2: i64 = 0;

    for line in input {
        let mut all_ints: Vec<i64> = line
            .split(" ")
            .map(|i| i.parse().unwrap())
            .collect();
        
        let mut all_last_digits: Vec<i64> =  vec![*all_ints.last().unwrap()];
        let mut all_first_digits: Vec<i64> = vec![*all_ints.first().unwrap()];
        while all_ints.iter().any(|&x| x != 0) {
            let new_ints: Vec<i64> = all_ints
                .windows(2)
                .map(|i| i[1] - i[0])
                .collect();
            all_last_digits.push( *new_ints.last().unwrap());
            all_first_digits.push(*new_ints.first().unwrap());
            all_ints = new_ints;
        }

        let temp: i64 = all_last_digits.iter().sum();
        sol1 += temp as u64;
        
        all_first_digits.reverse();
        let extrapolated_first_digits = all_first_digits
            .iter()
            .skip(1)
            .fold(all_first_digits[0], |acc, &curr| curr - acc);
        sol2 += extrapolated_first_digits;
    }

    (Solution::from(sol1), Solution::from(sol2 as u64))
}
