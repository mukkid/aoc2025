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
        Self { x, y, z }
    }
}

fn distance(first: &Junction, second: &Junction) -> f32 {
    ((((second.x - first.x) as i64).pow(2)
        + ((second.y - first.y) as i64).pow(2)
        + ((second.z - first.z) as i64).pow(2)) as f32)
        .sqrt()
}

fn create_junction_pairs(junctions: &[Junction]) -> Vec<(&Junction, &Junction, f32)> {
    let mut pairs = Vec::new();
    for j1 in junctions {
        for j2 in junctions {
            if j1 == j2 {
                continue;
            }
            let d = distance(&j1, &j2);
            if pairs.contains(&(j2, j1, d)) {
                continue;
            }
            pairs.push((j1, j2, d));
        }
    }
    pairs.sort_by(|p1, p2| p1.2.partial_cmp(&p2.2).unwrap());
    pairs
}

fn solve1(input_str: &str) -> usize {
    let mut circuits: Vec<HashSet<Junction>> = Vec::new();
    let unmapped_junctions: Vec<Junction> = input_str
        .lines()
        .map(|l| Junction::from_iter(l.splitn(3, ",").map(|c| c.parse::<i32>().unwrap())))
        .collect();
    let pairs = create_junction_pairs(&unmapped_junctions);
    let mut p_iter = pairs.into_iter();
    let (j1, j2, _) = p_iter.next().unwrap();
    let mut init = HashSet::new();
    init.insert(*j1);
    init.insert(*j2);
    circuits.push(init);
    let mut conn_count = 1;
    'outer: while let Some((j1, j2, d)) = p_iter.next() {
        if conn_count == 10 {
            break;
        }
        for hm in circuits.iter_mut() {
            if (hm.contains(j1) && !hm.contains(j2)) || (!hm.contains(j1) && hm.contains(j2)) {
                hm.insert(*j1);
                hm.insert(*j2);
                conn_count += 1;
                continue 'outer;
            } else if hm.contains(j1) && hm.contains(j2) {
                continue 'outer;
            }
        }
        let mut fresh_c = HashSet::new();
        fresh_c.insert(*j1);
        fresh_c.insert(*j2);
        circuits.push(fresh_c);
        conn_count += 1;
    }
    circuits.sort_by_key(|k| k.len());
    circuits.iter().rev().take(3).fold(1, |a, l| a * l.len())
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
