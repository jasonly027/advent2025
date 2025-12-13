use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn p1(input: &str) -> u32 {
    let mut adj_list: HashMap<String, HashSet<String>> = HashMap::new();
    let mut cache: HashMap<String, u32> = HashMap::new();

    let mappings = input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .expect("should have mapping delimiter")
        })
        .map(|(src, out_str)| {
            (
                src.to_string(),
                out_str
                    .split_whitespace()
                    .map(|o| o.to_string())
                    .collect_vec(),
            )
        });
    for (src, out) in mappings {
        adj_list.entry(src).or_default().extend(out);
    }

    dfs("you", "out", &adj_list, &mut cache)
}

fn dfs(
    src: &str,
    dest: &str,
    adj: &HashMap<String, HashSet<String>>,
    cache: &mut HashMap<String, u32>,
) -> u32 {
    if let Some(&n) = cache.get(src) {
        return n;
    }

    let count = if src == dest {
        1
    } else {
        adj[src].iter().map(|out| dfs(out, dest, adj, cache)).sum()
    };
    cache.insert(src.to_string(), count);

    count
}

pub fn p2(input: &str) -> u64 {
    let mut adj_list: HashMap<String, HashSet<String>> = HashMap::new();
    let mut cache: HashMap<(&str, bool, bool), u64> = HashMap::new();

    let mappings = input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .expect("should have mapping delimiter")
        })
        .map(|(src, out_str)| {
            (
                src.to_string(),
                out_str
                    .split_whitespace()
                    .map(|o| o.to_string())
                    .collect_vec(),
            )
        });
    for (src, out) in mappings {
        adj_list.entry(src).or_default().extend(out);
    }

    dfs2(("svr", false, false), "out", &adj_list, &mut cache)
}

fn dfs2<'a>(
    (src, dac, fft): (&'a str, bool, bool),
    dest: &str,
    adj: &'a HashMap<String, HashSet<String>>,
    cache: &mut HashMap<(&'a str, bool, bool), u64>,
) -> u64 {
    if let Some(&n) = cache.get(&(src, dac, fft)) {
        return n;
    }

    let count = if src == dest {
        if dac && fft { 1 } else { 0 }
    } else {
        adj[src]
            .iter()
            .map(|out| {
                dfs2(
                    (out, dac || out == "dac", fft || out == "fft"),
                    dest,
                    adj,
                    cache,
                )
            })
            .sum()
    };
    cache.insert((src, dac, fft), count);

    count
}

#[cfg(test)]
mod tests {
    use test_log::test;
    use tracing::info;

    use super::*;

    #[test]
    fn sample_p1() {
        assert_eq!(5, p1(include_str!("day11.sample.txt")));
    }

    #[test]
    fn test_p1() {
        let ans = p1(include_str!("day11.txt"));
        info!(ans);
    }

    #[test]
    fn sample_p2() {
        assert_eq!(2, p2(include_str!("day11.sample2.txt")));
    }

    #[test]
    fn test_p2() {
        let ans = p2(include_str!("day11.txt"));
        info!(ans);
    }
}
