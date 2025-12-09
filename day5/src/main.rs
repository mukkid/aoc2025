const INPUT_STR: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
                    n1.parse::<u64>().unwrap(),
                    n2.parse::<u64>().unwrap(),
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
    let out2 = solve2(INPUT_STR);
    println!("{out1}, {out2}");
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

fn solve2(input: &str) -> u64 {
    let mut ranges: Vec<Range> = vec![];
    let mut l_iter = input.lines();
    while let Some(line) = l_iter.next() {
        if line.is_empty() {
            break
        }
        let r = Range::from_string(line);
        ranges.push(r);
    }
    compress_ranges(ranges)
}

fn compress_ranges(ranges: Vec<Range>) -> u64 {
    let mut range = ranges.clone();
    'outer: loop {
        for (idx, first) in (0..range.len() - 1).enumerate() {
            for second in idx+1..range.len() {
                if range[first].contains(range[second].lower) {
                    range[first].upper = range[first].upper.max(range[second].upper);
                    range.remove(second);
                    continue 'outer
                } else if range[first].contains(range[second].upper) {
                    range[first].lower = range[first].lower.min(range[second].lower);
                    range.remove(second);
                    continue 'outer
                } else if range[second].contains(range[first].lower) {
                    range[second].upper = range[second].upper.max(range[first].upper);
                    range.remove(first);
                    continue 'outer
                } else if range[second].contains(range[first].upper) {
                    range[second].lower = range[second].lower.min(range[first].lower);
                    range.remove(first);
                    continue 'outer
                }
            }
        }
        return range.into_iter().map(|r| (r.upper - r.lower) + 1).sum::<u64>()
    }    
}

#[cfg(test)]
mod test {
    const TEST_STR: &str = include_str!("test.txt");
    #[test]
    fn part1() {
        let out = crate::solve1(TEST_STR);
        assert_eq!(out, 3)
    }

    #[test]
    fn part2() {
        let out = crate::solve2(TEST_STR);
        assert_eq!(out, 14);
    }
}
