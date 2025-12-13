use fxhash::FxHashSet;
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
        let mut states = FxHashSet::from_iter([self.lights]);

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

#[derive(Debug, Clone)]
struct Machine2 {
    current: Vec<u32>,
    buttons: Vec<Vec<usize>>,
    target: Vec<u32>,
}

impl Machine2 {
    fn from_str(str: &str) -> Self {
        let mut iter = str.split_whitespace().skip(1).peekable();

        let buttons = iter
            .peeking_take_while(|l| l.starts_with('('))
            // Strip brackets
            .map(|raw| &raw[1..raw.len() - 1])
            // Separate into num vec
            .map(|field| {
                field
                    .split(',')
                    .map(|b| {
                        b.parse::<usize>()
                            .expect("field in button group should be a number")
                    })
                    .collect_vec()
            })
            .collect_vec();

        let target_str = {
            let field = iter.next().expect("should have joltage target field");
            &field[1..field.len() - 1] // Strip brackets
        };

        let target = target_str
            .split(',')
            .map(|field| {
                field
                    .parse()
                    .expect("field in joltage target should be num")
            })
            .collect_vec();

        Self {
            current: vec![0; target.len()],
            buttons,
            target,
        }
    }

    fn solve(&self) -> u32 {
        let mut states = FxHashSet::from_iter([self.current.clone()]);

        let mut presses = 0;
        while !states.contains(&self.target) {
            let curr_states = states.drain().collect_vec();
            for (state, btn) in curr_states.iter().cartesian_product(self.buttons.iter()) {
                let mut new_state = state.clone();
                for &b in btn {
                    new_state[b] += 1;
                }

                if new_state
                    .iter()
                    .zip(&self.target)
                    .all(|(curr, target)| curr <= target)
                {
                    states.insert(new_state);
                }
            }
            presses += 1;
        }
        dbg!(presses)
    }
}

pub fn p2(input: &str) -> u32 {
    input
        .lines()
        .map(Machine2::from_str)
        .map(|m| m.solve())
        .sum()
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
        assert_eq!(33, p2(include_str!("day10.sample.txt")));
    }

    // TLE
    #[test]
    fn test_p2() {
        let ans = p2(include_str!("day10.txt"));
        info!(ans);
    }
}
