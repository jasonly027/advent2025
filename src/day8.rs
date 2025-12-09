use std::{
    collections::{HashMap, hash_map::Entry},
    mem::{self, swap},
};

use itertools::Itertools;

type Point = (u64, u64, u64);

pub fn p1(input: &str, connections: usize) -> usize {
    let points: Vec<Point> = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let combos: Vec<(&Point, &Point, f64)> = points
        .iter()
        .tuple_combinations()
        .map(|(p0, p1)| (p0, p1, euclid_dist(p0, p1)))
        .sorted_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .collect();

    let mut dsu = Dsu::new();
    for (p0, p1, _) in &combos[..connections] {
        dsu.unify(**p0, **p1);
    }

    dsu.size
        .into_values()
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .product()
}

fn euclid_dist((x0, y0, z0): &Point, (x1, y1, z1): &Point) -> f64 {
    let sum =
        ((x0.abs_diff(*x1)).pow(2) + (y0.abs_diff(*y1)).pow(2) + (z0.abs_diff(*z1)).pow(2)) as f64;
    sum.sqrt()
}

#[derive(Debug, Default)]
struct Dsu {
    parent: HashMap<Point, Point>,
    size: HashMap<Point, usize>,
}

impl Dsu {
    fn new() -> Self {
        Dsu::default()
    }

    fn find(&mut self, a: Point) -> Point {
        if let Entry::Vacant(e) = self.parent.entry(a) {
            e.insert(a);
            self.size.insert(a, 1);
        }
        let pa = *self.parent.get(&a).unwrap();

        if pa != a {
            let new_pa = self.find(pa);
            self.parent.insert(a, new_pa);
            new_pa
        } else {
            a
        }
    }

    fn unify(&mut self, a: Point, b: Point) {
        let mut pa = self.find(a);
        let mut pb = self.find(b);
        if pa == pb {
            return;
        }

        if self.size.get(&pb).unwrap() < self.size.get(&pa).unwrap() {
            swap(&mut pb, &mut pa);
        }

        self.parent.insert(pa, pb);

        let pa_size = mem::take(self.size.get_mut(&pa).unwrap());
        *self.size.get_mut(&pb).unwrap() += pa_size;
    }
}

pub fn p2(input: &str) -> u64 {
    let points: Vec<Point> = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let combos: Vec<(&Point, &Point, f64)> = points
        .iter()
        .tuple_combinations()
        .map(|(p0, p1)| (p0, p1, euclid_dist(p0, p1)))
        .sorted_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .collect();

    let mut dsu = Dsu::new();
    combos
        .iter()
        .take_while_inclusive(|(p0, p1, _)| {
            dsu.unify(**p0, **p1);

            let root = dsu.find(**p0);
            *dsu.size.get(&root).unwrap() != points.len()
        })
        .last()
        .map(|(p0, p1, _)| p0.0 * p1.0)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
    use tracing::info;

    #[test]
    fn sample_p1() {
        assert_eq!(40, p1(include_str!("day8.sample.txt"), 10));
    }

    #[test]
    fn test_p1() {
        let ans = p1(include_str!("day8.txt"), 1000);
        info!(ans);
    }

    #[test]
    fn sample_p2() {
        assert_eq!(25272, p2(include_str!("day8.sample.txt")));
    }

    #[test]
    fn test_p2() {
        let ans = p2(include_str!("day8.txt"));
        info!(ans);
    }
}
