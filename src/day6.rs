use std::ops::{self, Mul};

use tracing::info;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

pub fn p1(input: &str) -> u64 {
    let len = input.lines().next().unwrap().split_whitespace().count();
    let mut problems: Vec<(Vec<u64>, Option<Operation>)> = vec![Default::default(); len];

    for line in input.lines() {
        for (idx, field) in line.split_whitespace().enumerate() {
            if let Ok(num) = field.parse() {
                problems[idx].0.push(num);
                continue;
            }
            match field {
                "+" => problems[idx].1 = Some(Operation::Add),
                "*" => problems[idx].1 = Some(Operation::Multiply),
                _ => unreachable!("Should be + or *"),
            }
        }
    }

    problems.into_iter().fold(0, |total, (vals, op)| {
        total
            + match op {
                Some(Operation::Add) => vals.iter().sum(),
                Some(Operation::Multiply) => vals.iter().product(),
                None => 0,
            }
    })
}

pub fn p2(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map_while(|line| {
            if line.is_empty() {
                return None;
            }
            Some(line.chars().collect())
        })
        .collect();

    let mut res = 0;

    let mut vals: Vec<u64> = Vec::new();
    let mut op = Operation::Add;
    for c in 0..grid[0].len() {
        let mut is_divider = true;

        vals.push(0);
        for cell in grid.iter().map(|row| row[c]) {
            match cell {
                ' ' => {}
                '+' => op = Operation::Add,
                '*' => op = Operation::Multiply,
                x => {
                    let curr_val = vals.last_mut().unwrap();
                    *curr_val *= 10;
                    *curr_val += x.to_digit(10).unwrap() as u64;
                }
            }
            if cell != ' ' {
                is_divider = false;
            }
        }

        if is_divider {
            vals.pop();
            res += match op {
                Operation::Add => vals.iter().sum::<u64>(),
                Operation::Multiply => vals.iter().product(),
            };
            vals.clear();
        }
    }
    res += match op {
        Operation::Add => vals.iter().sum::<u64>(),
        Operation::Multiply => vals.iter().product(),
    };

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn sample_p1() {
        assert_eq!(4277556, p1(include_str!("day6.sample.txt")))
    }

    #[test]
    fn test_p1() {
        let ans = p1(include_str!("day6.txt"));
        info!(ans);
    }

    #[test]
    fn sample_p2() {
        assert_eq!(3263827, p2(include_str!("day6.sample.txt")))
    }

    #[test]
    fn test_p2() {
        let ans = p2(include_str!("day6.txt"));
        info!(ans);
    }
}
