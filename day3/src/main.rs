const INPUT_FILE: &str = include_str!("input.txt");

fn main() {
    let input_str = String::from(INPUT_FILE);
    let out = solve2(input_str);
    println!("{out}");
}

fn solve1(input: String) -> u32 {
    let mut joltage: Vec<u32> = Vec::new();
    for line in input.lines() {
        let tens_band = line.len() - 1;
        let (ones_band, tens) = line.as_bytes()[..tens_band].iter().enumerate().fold(
            (0, 0),
            |(best_i, best_v), (i, &v)| {
                if v > best_v { (i, v) } else { (best_i, best_v) }
            },
        );
        let ones = line.as_bytes()[ones_band + 1..].iter().max().unwrap();
        let tens = (tens - b'0') * 10;
        let ones = ones - b'0';
        let composite = ones + tens;
        joltage.push(composite as u32);
    }
    joltage.iter().sum::<u32>()
}

fn solve2(input: String) -> u64 {
    let mut joltages: Vec<u64> = Vec::new();
    for line in input.lines() {
        let mut joltage: u64 = 0;
        let mut start = 0;
        for band_width in (0..12).rev() {
            let o = largest_digit(line.as_bytes(), start, line.len() - band_width);
            start = o.0 + 1;
            joltage *= 10;
            joltage += o.1 as u64;
        }
        joltages.push(joltage);
    }
    joltages.iter().sum::<u64>()
}

fn largest_digit(nums: &[u8], start: usize, end: usize) -> (usize, u8) {
    let (idx, num) = nums[start..end]
        .iter()
        .enumerate()
        .fold((0, 0), |(best_i, best_v), (i, &v)| {
            if v > best_v {
                (i, v)
            } else {
                (best_i, best_v)
            }
        });
    (idx + start, num - b'0')
}

#[cfg(test)]
mod test {
    const TEST_FILE: &str = include_str!("test.txt");
    #[test]
    fn part1() {
        let input_str = String::from(TEST_FILE);
        let out = crate::solve1(input_str);
        assert_eq!(out, 357)
    }

    #[test]
    fn part1_sub_test() {
        let input_str = "5435113354324445355543423523324245133533362334234141663246333323544553332443432533433423343343453463".to_string();
        let out = crate::solve1(input_str);
        assert_eq!(out, 66);
    }

    #[test]
    fn part2() {
        let input_str = String::from(TEST_FILE);
        let out = crate::solve2(input_str);
        assert_eq!(out, 3121910778619)
    }
}
