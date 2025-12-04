fn main() {
    let input = String::from(include_str!("input.txt"));
    let test = "1234";
    dbg!(create_repeated_number(test, 2));
    let output = solve2(input);
    println!("{output}");
}

#[derive(Debug, Clone, Copy)]
struct Range {
    lower: i64,
    upper: i64,
}

fn solve(input: String) -> i64 {
    let invalids: i64 = input.trim()
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
    let invalids: i64 = input.trim()
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

fn create_repeated_number(num_as_string: &str, repititions: usize) -> Vec<String> {
    let mut nums: Vec<String> = Vec::new();
    let len = num_as_string.len();
    for i in 1..num_as_string.len() {
        let d = String::from(&num_as_string[0..i]);
        let mut num = String::new();
        num.push_str(&d.repeat(repititions));
        nums.push(num);
    }
    nums
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let input = String::from(include_str!("test.txt"));
        let output = crate::solve(input);
        let expected = 1227775554;
        assert_eq!(output, expected);
    }

    #[test]
    fn part2() {
        let input = String::from(include_str!("test.txt"));
        let output = crate::solve2(input);
        let expected = 4174379265;
        assert_eq!(output, expected);
    }
}
