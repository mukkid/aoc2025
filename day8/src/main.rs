use std::{arch::x86_64::_mm_unpackhi_pd, collections::HashSet};

const INPUT_STR: &str = include_str!("input.txt");

fn main() {
    let out1 = solve1(INPUT_STR);
    println!("{out1}");
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Junction {
    x: i32,
    y: i32,
    z: i32,
}

impl Junction {
    fn from_iter(mut iter: impl Iterator<Item = i32>) -> Self {
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();
        Self { x, y, z}
    }
}

fn distance(first: &Junction, second: &Junction) -> f32 {
    (((second.x - first.x).pow(2) + (second.y - first.y).pow(2) + (second.z - first.z).pow(2)) as f32).sqrt()
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Circuit {
    junctions: HashSet<Junction>
}

fn solve1(input_str: &str) -> i32 {
    let unmapped_junctions: Vec<Junction> = input_str.lines().map(|l| {
        Junction::from_iter(l.splitn(3, ",").map(|c| c.parse::<i32>().unwrap()))
    }).collect();

    let j_len = unmapped_junctions.len();
    let mut c = Circuit::default();
    
    for (idx, first) in (0..j_len - 1).enumerate() {
        let shortest = f32::MAX;
        for second in idx+1..j_len {
            if distance(&unmapped_junctions[first], &unmapped_junctions[second]) < shortest {
                c.junctions.insert(unmapped_junctions[first]);
                c.junctions.insert(unmapped_junctions[first]);
            }
        }
    }

    dbg!(c);
    todo!();
}

#[cfg(test)]
mod test {
    const TEST_STR: &str = include_str!("test.txt");

    #[test]
    fn part1() {
        let out = crate::solve1(TEST_STR);
        assert_eq!(out, 40);
    }
}
