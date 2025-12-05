pub fn p1(input: &str) -> i32 {
    fn is_accessible(r: usize, c: usize, grid: &[Vec<char>]) -> bool {
        const MAX_LOAD: i32 = 3;
        const DIRS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let papers_nearby = DIRS
            .iter()
            .filter_map(|(dr, dc)| Some((r.checked_add_signed(*dr)?, c.checked_add_signed(*dc)?)))
            .fold(0, |acc, (new_r, new_c)| {
                let cell = grid.get(new_r).and_then(|row| row.get(new_c));

                if matches!(cell, Some('@')) {
                    acc + 1
                } else {
                    acc
                }
            });

        papers_nearby <= MAX_LOAD
    }

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    (0..grid.len()).fold(0, |acc, r| {
        acc + (0..grid[r].len()).fold(0, |acc, c| {
            if grid[r][c] == '@' && is_accessible(r, c, &grid) {
                acc + 1
            } else {
                acc
            }
        })
    })
}

pub fn p2(input: &str) -> i32 {
    fn is_accessible(r: usize, c: usize, grid: &[Vec<char>]) -> bool {
        const MAX_LOAD: usize = 3;
        const DIRS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let papers_nearby = DIRS
            .iter()
            .filter_map(|(dr, dc)| Some((r.checked_add_signed(*dr)?, c.checked_add_signed(*dc)?)))
            .filter(|(new_r, new_c)| {
                let cell = grid.get(*new_r).and_then(|row| row.get(*new_c));
                matches!(cell, Some('@'))
            })
            .count();

        papers_nearby <= MAX_LOAD
    }

    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut ans = 0;
    loop {
        let pruned = (0..grid.len()).fold(0, |acc, r| {
            acc + (0..grid[r].len()).fold(0, |acc, c| {
                if grid[r][c] == '@' && is_accessible(r, c, &grid) {
                    grid[r][c] = '.';
                    acc + 1
                } else {
                    acc
                }
            })
        });
        if pruned == 0 {
            break;
        }
        ans += pruned;
    }

    ans
}

#[cfg(test)]
mod tests {
    use test_log::test;
    use tracing::info;

    use super::*;

    #[test]
    fn sample_p1() {
        assert_eq!(13, p1(include_str!("day4.sample.txt")))
    }

    #[test]
    fn test_p1() {
        let ans = p1(include_str!("day4.txt"));
        info!(ans);
    }

    #[test]
    fn sample_p2() {
        assert_eq!(43, p2(include_str!("day4.sample.txt")))
    }

    #[test]
    fn test_p2() {
        let ans = p2(include_str!("day4.txt"));
        info!(ans);
    }
}
