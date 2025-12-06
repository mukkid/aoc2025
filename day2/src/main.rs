fn main() {
    let input = String::from(include_str!("input.txt"));
    let output = solve2(input);
    println!("{output}");
    println!("DONE");
}

#[derive(Debug, Clone, Copy)]
struct Range {
    lower: i64,
    upper: i64,
}

fn solve(input: String) -> i64 {
    let invalids: i64 = input
        .trim()
        .split(",")
        .map(|c| {
            let (lower, upper) = c
                .split_once("-")
                .map(|(l, u)| (l.parse::<i64>().unwrap(), u.parse::<i64>().unwrap()))
                .unwrap();
            let r = Range { lower, upper };
            check_ranges(r)
        })
        .flatten()
        .sum();
    invalids
}

fn solve2(input: String) -> i64 {
    input
        .trim()
        .split(',')
        .map(|range| {
            let (lower, upper) = range
                .split_once('-')
                .map(|(l, u)| (l.parse::<i64>().unwrap(), u.parse::<i64>().unwrap()))
                .unwrap();
            (lower..=upper).filter(|&n| is_invalid(n)).sum::<i64>()
        })
        .sum()
}

fn is_invalid(n: i64) -> bool {
    let s = n.to_string();
    let len = s.len();

    for pattern_len in 1..=len / 2 {
        if len % pattern_len == 0 {
            let pattern = &s[..pattern_len];
            if s == pattern.repeat(len / pattern_len) {
                return true;
            }
        }
    }
    false
}

fn check_ranges(r: Range) -> Vec<i64> {
    let lower_bound: String = r.lower.to_string();
    let mut nums: Vec<i64> = Vec::new();
    let bound = (lower_bound.len() / 2).max(1);
    let half = &lower_bound[..bound];
    let mut half_num = half.parse::<i64>().unwrap();
    loop {
        let full = format!("{}{}", half_num, half_num).parse::<i64>().unwrap();
        if full < r.lower {
            half_num += 1;
            continue;
        }
        if full >= r.lower && full <= r.upper {
            nums.push(full);
            half_num += 1;
        } else {
            break;
        }
    }
    nums
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn part1() {
        let input = String::from(TEST_INPUT);
        let output = crate::solve(input);
        let expected = 1227775554;
        assert_eq!(output, expected);
    }

    #[test]
    fn part2() {
        let input = String::from(TEST_INPUT);
        let output = crate::solve2(input);
        let expected = 4174379265;
        assert_eq!(output, expected);
    }
}
