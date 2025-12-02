const INPUT: &str = include_str!("day2.txt");

pub fn p1() -> i64 {
    fn invalid_ids(a: i64, b: i64) -> impl Iterator<Item = i64> {
        let range = a..=b;

        range.filter(|num| {
            let id = num.to_string();
            if id.len() % 2 == 1 {
                return false;
            }

            let (left, right) = id.split_at(id.len() / 2);
            left == right
        })
    }

    let mut ans = 0;

    for line in INPUT.trim().split(',') {
        let (left, right) = line.split_once('-').unwrap();
        let start = left.parse().unwrap();
        let end = right.parse().unwrap();

        ans += invalid_ids(start, end).sum::<i64>();
    }

    ans
}

pub fn p2() -> i64 {
    fn invalid_ids(a: i64, b: i64) -> impl Iterator<Item = i64> {
        let range = a..=b;

        range.filter(|num| {
            let id = num.to_string();
            let length = id.len();
            let range = (1..=length / 2).filter(|i| length % i == 0);

            for len in range {
                if id[..len].repeat(length / len) == id {
                    return true;
                }
            }

            false
        })
    }

    let mut ans = 0;

    for line in INPUT.trim().split(',') {
        let (left, right) = line.split_once('-').unwrap();
        let start = left.parse().unwrap();
        let end = right.parse().unwrap();

        ans += invalid_ids(start, end).sum::<i64>();
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

    #[test]
    fn p2_one_line() {
        let ans = include_str!("day2.txt")
            .trim()
            .split(',')
            .fold(0, |acc, line| {
                acc + line
                    .split_once('-')
                    .map(|(l, r)| {
                        (l.parse::<i64>().unwrap()..=r.parse::<i64>().unwrap())
                            .map(|n| n.to_string())
                            .filter(|ns| {
                                (1..=ns.len() / 2).any(|i| {
                                    ns.len() % i == 0 && ns[..i].repeat(ns.len() / i) == *ns
                                })
                            })
                            .map(|ns| ns.parse::<i64>().unwrap())
                            .sum::<i64>()
                    })
                    .unwrap()
            });

        println!("{ans}");
    }
}
