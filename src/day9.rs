use std::cmp::{max, min};

use itertools::Itertools;

type Point = (usize, usize);

pub fn p1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|x| x.parse().unwrap())
                .collect_tuple::<Point>()
                .unwrap()
        })
        .tuple_combinations()
        .map(|(p0, p1)| area(&p0, &p1))
        .max()
        .unwrap()
}

fn area((x0, y0): &Point, (x1, y1): &Point) -> usize {
    (x0.abs_diff(*x1) + 1) * (y0.abs_diff(*y1) + 1)
}

pub fn p2(input: &str) -> usize {
    let mut grid: Vec<Vec<bool>> = vec![vec![false; 99000]; 99000];

    let points: Vec<Point> = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|x| x.parse().unwrap())
                .collect_tuple::<Point>()
                .unwrap()
        })
        // Set corners
        .inspect(|&(x, y)| {
            grid[y][x] = true;
        })
        .collect();

    // Set borders
    for (&(x0, y0), &(x1, y1), &(x2, y2)) in points.iter().circular_tuple_windows().step_by(2) {
        // A vertical border
        debug_assert_eq!(x0, x1);
        grid[min(y0, y1)..=max(y0, y1)]
            .iter_mut()
            .for_each(|row| row[x0] = true);

        // A horizontal border
        debug_assert_eq!(y1, y2);
        grid[y1][min(x1, x2)..=max(x1, x2)]
            .iter_mut()
            .for_each(|c| *c = true);
    }

    points
        .iter()
        .tuple_combinations()
        .filter(|&(&p0, &p1)| is_within(p0, p1, &grid))
        .map(|(p0, p1)| area(p0, p1))
        .max()
        .unwrap()
}

fn is_within(mut p0: Point, mut p1: Point, grid: &[Vec<bool>]) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
    use tracing::info;

    #[test]
    fn sample_p1() {
        assert_eq!(50, p1(include_str!("day9.sample.txt")));
    }

    #[test]
    fn test_p1() {
        let ans = p1(include_str!("day9.txt"));
        info!(ans);
    }

    #[test]
    fn sample_p2() {
        assert_eq!(24, p2(include_str!("day9.sample.txt")));
    }

    #[test]
    fn test_p2() {
        let ans = p2(include_str!("day9.txt"));
        info!(ans);
    }
}
