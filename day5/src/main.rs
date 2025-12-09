const INPUT_STR: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy)]
struct Range {
    lower: u64,
    upper: u64,
}

impl Range {
    fn from_string<S: AsRef<str>>(s: S) -> Self {
        let (n1, n2) = s
            .as_ref()
            .trim()
            .split_once("-")
            .map(|(n1, n2)| {
                (
                    n1.parse::<u64>().unwrap_or(0),
                    n2.parse::<u64>().unwrap_or(0),
                )
            })
            .unwrap();
        Self {
            lower: n1,
            upper: n2,
        }
    }

    fn contains(&self, val: u64) -> bool {
        val >= self.lower && val <= self.upper
    }
}

fn main() {
    let out1 = solve1(INPUT_STR);
    println!("{out1}");
}

fn solve1(input: &str) -> u32 {
    let mut ranges = vec![];
    let mut l_iter = input.lines();
    let mut count = 0;
    while let Some(line) = l_iter.next() {
        if line.is_empty() {
            break
        }
        let r = Range::from_string(line);
        ranges.push(r);
    }

    while let Some(n) = l_iter.next() {
        let n: u64 = n.parse().unwrap();
        for range in ranges.iter() {
            if range.contains(n) {
                count += 1;
                break
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    const TEST_STR: &str = include_str!("test.txt");
    #[test]
    fn part1() {
        let out = crate::solve1(TEST_STR);
        assert_eq!(out, 3)
    }
}
