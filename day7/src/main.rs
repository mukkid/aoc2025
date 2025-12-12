use num_bigint::BigUint;
use num_traits::Num;

const INPUT_STR: &str = include_str!("input.txt");

fn main() {
    let out1 = solve1(INPUT_STR);
    println!("Part 1: {out1}");
    let out2 = solve2(INPUT_STR);
    println!("Part 2: {out2}");
}

fn solve1(input_str: &str) -> u64 {
    let mut nums = vec![];
    for line in input_str.lines() {
        if line.chars().all(|c| c == '.') {
            continue;
        }
        let binstr: String = line
            .chars()
            .map(|c| if c == '.' { '0' } else { '1' })
            .collect();
        let bignum = BigUint::from_str_radix(&binstr, 2).unwrap();
        nums.push(bignum);
    }
    collapse(nums)
}

fn solve2(input_str: &str) -> u64 {
    let mut nums = vec![];
    for line in input_str.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let binstr: String = line
            .chars()
            .map(|c| if c == '.' { '0' } else { '1' })
            .collect();
        if binstr.chars().all(|c| c == '0') {
            continue;
        }
        let bignum = BigUint::from_str_radix(&binstr, 2).unwrap();
        nums.push(bignum);
    }
    collapse2(nums)
}

fn collapse2(nums: Vec<BigUint>) -> u64 {
    let mut n_iter = nums.into_iter();
    let start = n_iter.next().unwrap();

    let mut path_counts = vec![BigUint::from(0u32); 141];

    for i in 0..141 {
        if start.bit(i as u64) {
            path_counts[i] = BigUint::from(1u32);
            break;
        }
    }

    for transform in n_iter {
        let mut new_counts = vec![BigUint::from(0u32); 141];

        for i in 0..141 {
            if path_counts[i] > BigUint::from(0u32) {
                if transform.bit(i as u64) {
                    if i > 0 {
                        new_counts[i - 1] += &path_counts[i];
                    }
                    if i < 140 {
                        new_counts[i + 1] += &path_counts[i];
                    }
                } else {
                    new_counts[i] += &path_counts[i];
                }
            }
        }

        path_counts = new_counts;
    }

    path_counts
        .iter()
        .filter(|&count| *count > BigUint::from(0u32))
        .map(|count| count.to_string().parse::<u64>().unwrap_or(0))
        .sum()
}

fn collapse(nums: Vec<BigUint>) -> u64 {
    let mut n_iter = nums.into_iter();
    let bit_width = 141;
    let mut count = 0;
    let start = n_iter.next().unwrap();
    let _ = n_iter.fold(start, |incoming, transform| {
        let split_b = incoming.clone() & transform;
        count += split_b.count_ones();
        let after_s: BigUint = (split_b.clone() << 1) | (split_b.clone() >> 1);
        let mask = (BigUint::from(1u32) << bit_width) - 1u32;
        let inverted_split_b = &split_b ^ &mask;
        let unsplit = inverted_split_b & incoming;
        after_s | unsplit
    });
    count
}

#[cfg(test)]
mod test {
    const TEST_STR: &str = include_str!("test.txt");
    #[test]
    fn proof_of_concept() {
        let current = 0b0000_0010_0010_0110;
        let blocker = 0b0100_0010_0000_0010;
        let final_n = 0b0000_0101_0010_0101;

        let split_b = current & blocker;
        let after_s = (split_b << 1) | (split_b >> 1);
        let unsplit = !split_b & current;
        let final_b = after_s | unsplit;
        assert_eq!(final_b, final_n);
    }

    #[test]
    fn part1() {
        let out = crate::solve1(TEST_STR);
        assert_eq!(out, 21)
    }

    #[test]
    fn part2() {
        let out = crate::solve2(TEST_STR);
        assert_eq!(out, 40);
    }
}
