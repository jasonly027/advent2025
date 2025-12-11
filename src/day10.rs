use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug)]
struct Machine {
    lights: u32,
    target: u32,
    buttons: Vec<u32>,
}

impl Machine {
    fn from_str(str: &str) -> Self {
        let mut iter = str.split_whitespace().peekable();

        let target_str = {
            let field = iter.next().expect("should have indicator lights field");
            &field[1..field.len() - 1] // Strip brackets
        };

        let light_cnt = target_str.len();

        let target = target_str.char_indices().fold(0, |target, (idx, c)| {
            let bit = match c {
                '#' => 1,
                '.' => 0,
                _ => unreachable!(),
            };
            target | (bit << (light_cnt - 1 - idx))
        });

        let buttons = iter
            .peeking_take_while(|l| l.starts_with('('))
            // Strip brackets
            .map(|raw| &raw[1..raw.len() - 1])
            // Separate into nums
            .map(|field| {
                field.split(',').map(|b| {
                    b.parse::<usize>()
                        .expect("field in button group should be a number")
                })
            })
            // To bit filter
            .map(|nums| nums.fold(0, |buttons, x| buttons | (1u32 << (light_cnt - 1 - x))))
            .collect_vec();

        Self {
            lights: 0,
            target,
            buttons,
        }
    }

    fn solve(&self) -> u32 {
        let mut states = HashSet::from([self.lights]);

        let mut presses = 0;
        while !states.contains(&self.target) {
            let curr_states = states.iter().cloned().collect_vec();
            for (state, btn) in curr_states.iter().cartesian_product(self.buttons.iter()) {
                states.insert(state ^ btn);
            }
            presses += 1;
        }
        presses
    }
}

pub fn p1(input: &str) -> u32 {
    input
        .lines()
        .map(Machine::from_str)
        .map(|m| m.solve())
        .sum()
}

pub fn p2(_input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use test_log::test;
    use tracing::info;

    use super::*;

    #[test]
    fn sample_p1() {
        assert_eq!(7, p1(include_str!("day10.sample.txt")));
    }

    #[test]
    fn test_p1() {
        let ans = p1(include_str!("day10.txt"));
        info!(ans);
    }

    #[test]
    fn sample_p2() {
        assert_eq!(7, p2(include_str!("day10.sample.txt")));
    }

    #[test]
    fn test_p2() {
        let ans = p2(include_str!("day10.txt"));
        info!(ans);
    }
}
