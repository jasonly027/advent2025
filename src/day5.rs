use std::{
    cmp::{Ordering, max},
    ops::RangeInclusive,
};

pub fn p1(input: &str) -> u64 {
    let mut lines = input.lines();

    let fresh_ranges: Vec<RangeInclusive<u64>> = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|line| {
            line.split_once("-")
                .map(|(left, right)| left.parse().unwrap()..=right.parse().unwrap())
                .unwrap()
        })
        .collect();

    lines
        .map(|l| l.parse::<u64>().unwrap())
        .filter(|id| fresh_ranges.iter().any(|r| r.contains(id)))
        .count() as u64
}

pub fn p2(input: &str) -> u64 {
    let mut ranges: Vec<RangeInclusive<u64>> = input
        .lines()
        .take_while(|l| !l.is_empty())
        .filter_map(|line| {
            line.split_once("-")
                .and_then(|(left, right)| Some((left.parse().ok()?)..=right.parse().ok()?))
        })
        .collect();

    ranges.sort_by(|a, b| (a.start(), a.end()).cmp(&(b.start(), b.end())));

    let mut sparse_range: Vec<RangeInclusive<u64>> = Vec::new();
    for range in ranges {
        let Some(prev) = sparse_range.last() else {
            sparse_range.push(range);
            continue;
        };

        let (prev_start, prev_end) = (*prev.start(), *prev.end());

        match prev_end.cmp(range.start()) {
            Ordering::Less => {
                sparse_range.push(range);
            }
            Ordering::Equal | Ordering::Greater => {
                sparse_range.pop();
                sparse_range.push(prev_start..=max(prev_end, *range.end()));
            }
        }
    }

    sparse_range
        .iter()
        .fold(0, |count, r| count + (r.end() - r.start() + 1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
    use tracing::info;

    #[test]
    fn sample_p1() {
        assert_eq!(3, p1(include_str!("day5.sample.txt")))
    }

    #[test]
    fn test_p1() {
        let ans = p1(include_str!("day5.txt"));
        info!(ans);
    }

    #[test]
    fn sample_p2() {
        assert_eq!(14, p2(include_str!("day5.sample.txt")))
    }

    #[test]
    fn test_p2() {
        let ans = p2(include_str!("day5.txt"));
        info!(ans);
    }
}
