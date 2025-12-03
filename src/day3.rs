use std::cmp;

const INPUT: &str = include_str!("day3.txt");

pub fn p1() -> u32 {
    fn joltage(bank: &[u32]) -> u32 {
        // Suffix max of bank with the first element omitted
        let mut suffix_max = Vec::with_capacity(bank.len());
        bank.iter().skip(1).rev().for_each(|curr| {
            let prev = suffix_max.last();
            let max = *cmp::max(curr, prev.unwrap_or(curr));
            suffix_max.push(max);
        });
        suffix_max.reverse();

        bank.iter()
            .zip(suffix_max)
            .map(|(x, largest_after)| x * 10 + largest_after)
            .max()
            .unwrap()
    }

    INPUT
        .lines()
        .map(|line| {
            let batteries = line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            joltage(&batteries)
        })
        .sum()
}

pub fn p2() -> u64 {
    const SEQUENCE_LEN: usize = 12;

    fn joltage(bank: &[u64]) -> u64 {
        let mut ans = Vec::with_capacity(SEQUENCE_LEN);

        let mut idx = 0;
        // Build answer digit-by-digit by choosing the first of the largest digit
        // in the range.
        //
        // It's always better to make the left most digits of the
        // answer the greatest because they are the most significant.
        //
        // We do not search past (bank.len() - SEQUENCE_LEN + ans.len()) because
        // there won't be enough remaining digits to build the answer.
        while ans.len() < SEQUENCE_LEN {
            let (val, chosen_idx) = (idx..=(bank.len() - SEQUENCE_LEN + ans.len()))
                .map(|i| (bank[i], i))
                .max_by(|a, b| {
                    if a.0 != b.0 {
                        a.0.cmp(&b.0)
                    } else {
                        b.1.cmp(&a.1)
                    }
                })
                .unwrap();

            ans.push(val);
            idx = chosen_idx + 1;
        }

        ans.iter().fold(0, |acc, i| acc * 10 + i)
    }

    INPUT
        .lines()
        .map(|line| {
            let batteries = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<_>>();

            joltage(&batteries)
        })
        .sum()
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
