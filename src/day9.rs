use std::{
    cmp::{max, min},
    ops::Not,
};

use itertools::{Itertools, chain};

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
        if x0 == x1 && y1 == y2 {
            // Vertical
            grid[min(y0, y1)..=max(y0, y1)]
                .iter_mut()
                .for_each(|row| row[x0] = true);
            // Horizontal
            grid[y1][min(x1, x2)..=max(x1, x2)]
                .iter_mut()
                .for_each(|c| *c = true);
        } else if y0 == y1 && x1 == x2 {
            // Horizontal
            grid[y0][min(x0, x1)..=max(x0, x1)]
                .iter_mut()
                .for_each(|c| *c = true);
            // Vertical
            grid[min(y1, y2)..=max(y1, y2)]
                .iter_mut()
                .for_each(|row| row[x1] = true);
        } else {
            unreachable!("Points are not adjacent");
        }
    }

    points
        .iter()
        .tuple_combinations()
        .map(|(p0, p1)| (p0, p1, area(p0, p1)))
        .sorted_by(|p0, p1| p1.2.cmp(&p0.2))
        .find(|&(&p0, &p1, _)| is_within(p0, p1, &grid))
        .unwrap()
        .2
}

fn is_within((x0, y0): Point, (x1, y1): Point, grid: &[Vec<bool>]) -> bool {
    let (x0, x1) = (min(x0, x1), max(x0, x1));
    let (y0, y1) = (min(y0, y1), max(y0, y1));

    let top = grid[y0 + 1][x0 + 1..x1].iter();
    let bottom = grid[y1 - 1][x0 + 1..x1].iter();
    let left = grid[y0 + 1..y1].iter().map(|row| &row[x0 + 1]);
    let right = grid[y0 + 1..y1].iter().map(|row| &row[x1 - 1]);

    chain!(top, bottom, left, right).all(|c| c.not())
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
