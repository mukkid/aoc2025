const INPUT_STR: &str = include_str!("input.txt");

fn main() {
    let out1 = solve1(INPUT_STR, 1000);
    println!("{out1}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

fn create_junction_pairs(junctions: &[Junction]) -> Vec<(usize, usize, f32)> {
    let mut pairs = Vec::new();
    for (idx, j1) in junctions.iter().enumerate() {
        for (idx2, j2) in junctions[idx + 1..].iter().enumerate() {
            let d = distance(&j1, &j2);
            pairs.push((idx, idx + idx2 + 1, d));
        }
    }
    pairs.sort_by(|p1, p2| p1.2.partial_cmp(&p2.2).unwrap());
    pairs
}

fn find(parent: &mut Vec<usize>, x: usize) -> usize {
    if parent[x] != x {
        parent[x] = find(parent, parent[x]);
    }
    parent[x]
}

fn union(parent: &mut Vec<usize>, size: &mut Vec<usize>, x: usize, y: usize) {
    let px = find(parent, x);
    let py = find(parent, y);
    if px == py {
        return;
    }
    parent[px] = py;
    size[py] += size[px];
}

fn solve1(input_str: &str, cap: usize) -> usize {
    let junctions: Vec<Junction> = input_str
        .lines()
        .map(|l| Junction::from_iter(l.splitn(3, ",").map(|c| c.parse::<i32>().unwrap())))
        .collect();

    let pairs = create_junction_pairs(&junctions);

    let mut parent: Vec<usize> = (0..junctions.len()).collect();
    let mut size: Vec<usize> = vec![1; junctions.len()];

    for (idx1, idx2, _) in pairs.iter().take(cap) {
        union(&mut parent, &mut size, *idx1, *idx2);
    }

    let mut component_sizes = Vec::new();
    for i in 0..parent.len() {
        if parent[i] == i {
            component_sizes.push(size[i]);
        }
    }

    component_sizes.sort_by(|a, b| b.cmp(a));
    component_sizes.iter().take(3).product()
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
