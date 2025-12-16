use std::collections::HashSet;

const INPUT_STR: &str = include_str!("input.txt");

fn main() {
    let out1 = solve1(INPUT_STR, 1000);
    println!("{out1}");
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Junction {
    x: i32,
    y: i32,
    z: i32,
    circuit_id: Option<usize>,
}

impl Junction {
    fn from_iter(mut iter: impl Iterator<Item = i32>) -> Self {
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();
        Self {
            x,
            y,
            z,
            circuit_id: None,
        }
    }
}

fn distance(first: &Junction, second: &Junction) -> f32 {
    ((((second.x - first.x) as i64).pow(2)
        + ((second.y - first.y) as i64).pow(2)
        + ((second.z - first.z) as i64).pow(2)) as f32)
        .sqrt()
}

fn create_junction_pairs(junctions: &[Junction]) -> Vec<(usize, usize, f32)> {
    let mut pairs = Vec::new();
    for (idx, j1) in junctions.iter().enumerate() {
        for (idx2, j2) in junctions[idx + 1..].iter().enumerate() {
            let d = distance(&j1, &j2);
            pairs.push((idx, idx + 1 + idx2, d));
        }
    }
    pairs.sort_by(|p1, p2| p1.2.partial_cmp(&p2.2).unwrap());
    pairs
}

fn solve1(input_str: &str, cap: usize) -> usize {
    let mut circuits: Vec<HashSet<Junction>> = Vec::new();
    let mut unmapped_junctions: Vec<Junction> = input_str
        .lines()
        .map(|l| Junction::from_iter(l.splitn(3, ",").map(|c| c.parse::<i32>().unwrap())))
        .collect();
    let pairs = create_junction_pairs(&unmapped_junctions);
    let mut p_iter = pairs.into_iter();
    let mut conn_count = 0;
    while let Some((idx1, idx2, _)) = p_iter.next() {
        if conn_count >= cap {
            break;
        }
        match (
            unmapped_junctions[idx1].circuit_id,
            unmapped_junctions[idx2].circuit_id,
        ) {
            (Some(i1), Some(i2)) => {
                if i1 != i2 {
                    unmapped_junctions[idx2].circuit_id = Some(i1);
                    unmapped_junctions.iter_mut().for_each(|j| {
                        if j.circuit_id == Some(i2) {
                            j.circuit_id = Some(i1);
                            circuits[i1].insert(*j);
                            circuits[i2].remove(j);
                        }
                    });
                    conn_count += 1;
                }
            }
            (Some(circuit_idx), None) => {
                unmapped_junctions[idx2].circuit_id = Some(circuit_idx);
                circuits[circuit_idx].insert(unmapped_junctions[idx2]);
                conn_count += 1;
            }
            (None, Some(circuit_idx)) => {
                unmapped_junctions[idx1].circuit_id = Some(circuit_idx);
                circuits[circuit_idx].insert(unmapped_junctions[idx1]);
                conn_count += 1;
            }
            (None, None) => {
                let circuit_idx = circuits.len();
                unmapped_junctions[idx1].circuit_id = Some(circuit_idx);
                unmapped_junctions[idx2].circuit_id = Some(circuit_idx);
                let mut fresh_c = HashSet::new();
                fresh_c.insert(unmapped_junctions[idx1]);
                fresh_c.insert(unmapped_junctions[idx2]);
                circuits.push(fresh_c);
                conn_count += 1;
            }
        }
    }
    circuits.sort_by_key(|k| k.len());
    circuits.iter().rev().take(3).fold(1, |a, l| a * l.len())
}

#[cfg(test)]
mod test {
    const TEST_STR: &str = include_str!("test.txt");

    #[test]
    fn part1() {
        let out = crate::solve1(TEST_STR, 10);
        assert_eq!(out, 40);
    }
}
