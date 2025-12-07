pub fn p1(input: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let mut res = 0;

    let coords = {
        let rows = grid.len();
        let cols = grid[0].len();
        (0..rows).flat_map(move |r| (0..cols).map(move |c| (r, c)))
    };
    for (r, c) in coords {
        let cell = grid[r][c];
        if cell == 'S' {
            let Some(next_sp) = grid.get_mut(r + 1).and_then(|row| row.get_mut(c)) else {
                continue;
            };
            match next_sp {
                '.' => *next_sp = 'S',
                '^' => {
                    res += 1;

                    let mut next_row = grid.get_mut(r + 1);
                    if let Some(left_sp) = next_row.as_mut().and_then(|row| row.get_mut(c - 1)) {
                        *left_sp = 'S';
                    }
                    if let Some(right_sp) = next_row.and_then(|row| row.get_mut(c + 1)) {
                        *right_sp = 'S';
                    }
                }
                _ => {}
            }
        }
    }

    res
}

enum Cell {
    Particle(u64),
    Split,
    Empty,
}

trait Beam {
    fn beam(&mut self, r: usize, c: usize, val: u64);
}

impl Beam for Vec<Vec<Cell>> {
    fn beam(&mut self, r: usize, c: usize, val: u64) {
        let Some(cell) = self.get_mut(r).and_then(|r| r.get_mut(c)) else {
            return;
        };
        match cell {
            Cell::Particle(c) => *c += val,
            Cell::Empty => *cell = Cell::Particle(val),
            Cell::Split => unreachable!("Beaming a split"),
        }
    }
}

pub fn p2(input: &str) -> u64 {
    let mut grid: Vec<Vec<Cell>> = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'S' => Cell::Particle(1),
                    '^' => Cell::Split,
                    '.' => Cell::Empty,
                    c => unreachable!("Unexpected char {c}"),
                })
                .collect()
        })
        .collect();

    let coords = {
        let rows = grid.len();
        let cols = grid[0].len();
        (0..rows).flat_map(move |r| (0..cols).map(move |c| (r, c)))
    };
    for (r, c) in coords {
        let Cell::Particle(cell) = grid[r][c] else {
            continue;
        };
        let Some(next_cell) = grid.get_mut(r + 1).and_then(|row| row.get_mut(c)) else {
            continue;
        };
        match next_cell {
            Cell::Particle(_) | Cell::Empty => grid.beam(r + 1, c, cell),
            Cell::Split => {
                grid.beam(r + 1, c - 1, cell);
                grid.beam(r + 1, c + 1, cell);
            }
        }
    }

    grid.last()
        .map(|row| {
            row.iter().fold(0, |acc, c| {
                if let Cell::Particle(c) = c {
                    acc + c
                } else {
                    acc
                }
            })
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use test_log::test;
    use tracing::info;

    use super::*;

    #[test]
    fn sample_p1() {
        assert_eq!(21, p1(include_str!("day7.sample.txt")));
    }

    #[test]
    fn test_p1() {
        let ans = p1(include_str!("day7.txt"));
        info!(ans);
    }

    #[test]
    fn sample_p2() {
        assert_eq!(40, p2(include_str!("day7.sample.txt")));
    }

    #[test]
    fn test_p2() {
        let ans = p2(include_str!("day7.txt"));
        info!(ans);
    }
}
