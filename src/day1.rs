use std::cmp::Ordering;

const INPUT: &str = include_str!("day1.txt");
const INTIAL_POS: i32 = 50;
const POS_CEIL: i32 = 100;

pub fn p1() -> i32 {
    let mut dial = INTIAL_POS;

    let mut ans = 0;
    for line in INPUT.lines() {
        let (dir, shift) = {
            let (dir, shift) = line.split_at(1);
            (dir, shift.parse::<i32>().unwrap())
        };

        if dir == "L" {
            dial -= shift;
        } else {
            dial += shift;
        }
        dial = ((dial % POS_CEIL) + POS_CEIL) % POS_CEIL;

        if dial == 0 {
            ans += 1;
        }
    }

    ans
}

pub fn p2() -> i32 {
    let mut dial = INTIAL_POS;

    let mut ans = 0;
    for line in INPUT.lines() {
        let (dir, shift) = {
            let (dir, shift) = line.split_at(1);
            (dir, shift.parse::<i32>().unwrap())
        };

        if dir == "L" {
            let did_start_from_zero = dial == 0;
            dial -= shift;

            match dial.cmp(&0) {
                Ordering::Less => {
                    if !did_start_from_zero {
                        ans += 1;
                    }
                    ans += dial.abs() / POS_CEIL;
                }
                Ordering::Equal => ans += 1,
                _ => {}
            }
        } else {
            dial += shift;

            if dial >= POS_CEIL {
                ans += dial / POS_CEIL;
            }
        }

        dial = ((dial % POS_CEIL) + POS_CEIL) % POS_CEIL;
    }

    ans
}

#[cfg(test)]
mod tests {
    #[test]
    fn p1() {
        let ans = super::p1();
        println!("{ans}");
    }

    #[test]
    fn p2() {
        let ans = super::p2();
        println!("{ans}");
    }
}
