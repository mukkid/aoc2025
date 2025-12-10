use ndarray::{Array1, Array2, Axis, concatenate, s};

const INPUT_STR: &str = include_str!("input.txt");
const INPUT_BYTES: &[u8] = include_bytes!("input.txt");

fn main() {
    let out1 = solve1(INPUT_STR);
    let out2 = solve2(INPUT_BYTES);
    println!("{out1}, {out2}");
}

#[derive(Debug, Copy, Clone)]
enum CentiMath {
    Number(u64),
    Operator(CentiOperator),
    Blank
}

#[derive(Debug, Copy, Clone)]
enum CentiOperator {
    Mul,
    Add,
}

fn transpose<T: Copy>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..matrix[0].len())
        .map(|col| matrix.iter().map(|row| row[col]).collect())
        .collect()
}

fn split_by_space_rows(arr: Array2<&u8>) -> Vec<Array2<&u8>> {
    let mut splits = Vec::new();
    let mut start = 0;

    for i in 0..arr.nrows() {
        if arr.row(i).iter().all(|&c| *c == b' ') {
            if i > start {
                splits.push(arr.slice(s![start..i, ..]).to_owned());
            }
            start = i + 1;
        }
    }

    if start < arr.nrows() {
        splits.push(arr.slice(s![start.., ..]).to_owned());
    }

    splits
}

fn solve1(input_str: &str) -> u64 {
    let input_grids: Vec<Vec<CentiMath>> = input_str
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|sp| match sp.trim().parse::<u64>() {
                    Ok(n) => CentiMath::Number(n),
                    Err(_) => match sp {
                        "*" => CentiMath::Operator(CentiOperator::Mul),
                        "+" => CentiMath::Operator(CentiOperator::Add),
                        _ => unreachable!(),
                    },
                })
                .collect()
        })
        .collect();
    let t_grid = transpose(input_grids);
    t_grid.into_iter().map(|v| compress(v)).sum()
}

fn solve2(input_bytes: &[u8]) -> u64 {
    let width = input_bytes.iter().position(|&c| c == b'\n').unwrap();
    let height = input_bytes.iter().filter(|&c| *c == b'\n').count();
    let matrix = Array2::from_shape_vec(
        (height, width),
        Vec::from_iter(input_bytes.iter().filter(|&c| *c != b'\n')),
    )
    .unwrap();
    let t_mat = matrix.t();
    let mut total = 0;
    for row in split_by_space_rows(t_mat.to_owned()) {
        let mut nums = Vec::new();
        let mut op = CentiOperator::Add;
        for r in row.rows() {
            let m: Vec<_> = r.iter().map(|&c| match *c {
                48..=57 => CentiMath::Number((c - b'0') as u64),
                42 => CentiMath::Operator(CentiOperator::Mul),
                43 => CentiMath::Operator(CentiOperator::Add),
                32 => CentiMath::Blank,
                _ => unreachable!()
            }).rev().collect();
            if let CentiMath::Operator(o) = &m[0] {
                op = *o;
            }
            let val = &m[1..].iter().rev().fold(0, |acc, e| {
                match e {
                    CentiMath::Number(n) => acc * 10 + n,
                    CentiMath::Blank => acc,
                    _ => unreachable!()
                }
            });
            nums.push(val.clone());
        }
        let v = match op {
            CentiOperator::Mul => nums.into_iter().fold(1, |acc, n| acc*n),
            CentiOperator::Add => nums.into_iter().fold(0, |acc, n| acc+n)
        };
        total += v;
    }
    total
}

fn compress(input_vec: Vec<CentiMath>) -> u64 {
    let mut v_iter = input_vec.iter().rev();
    let Some(CentiMath::Operator(o)) = v_iter.next() else {
        panic!("not an operator")
    };
    match o {
        CentiOperator::Mul => v_iter.fold(1, |acc, &n| {
            if let CentiMath::Number(num) = n {
                acc * num
            } else {
                panic!("not a number")
            }
        }),
        CentiOperator::Add => v_iter.fold(0, |acc, &n| {
            if let CentiMath::Number(num) = n {
                acc + num
            } else {
                panic!("not a number")
            }
        }),
    }
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &str = include_str!("test.txt");
    const TEST_BYTES: &[u8] = include_bytes!("test.txt");
    #[test]
    fn part1() {
        let out = crate::solve1(TEST_INPUT);
        assert_eq!(out, 4277556);
    }

    #[test]
    fn part2() {
        let out = crate::solve2(TEST_BYTES);
        assert_eq!(out, 3263827);
    }
}
