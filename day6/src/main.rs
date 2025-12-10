const INPUT_STR: &str = include_str!("input.txt");

fn main() {
    let out1 = solve1(INPUT_STR);
    println!("{out1}");
}

#[derive(Debug, Copy, Clone)]
enum CentiMath {
    Number(u64),
    Operator(CentiOperator)
}

#[derive(Debug, Copy, Clone)]
enum CentiOperator {
    Mul,
    Add,
}


fn transpose<'a>(matrix: Vec<Vec<CentiMath>>) -> Vec<Vec<CentiMath>> {
    (0..matrix[0].len())
        .map(|col| matrix.iter().map(|row| row[col]).collect())
        .collect()
}

fn solve1(input_str: &str) -> u64 {
    let input_grids: Vec<Vec<CentiMath>> = input_str.lines().map(|s| {
        s.split_whitespace().map(|sp|
        match sp.trim().parse::<u64>() {
            Ok(n) => CentiMath::Number(n),
            Err(_) => match sp {
                "*" => CentiMath::Operator(CentiOperator::Mul),
                "+" => CentiMath::Operator(CentiOperator::Add),
                _ => unreachable!()
            },
        }).collect()
    }).collect();
    let t_grid = transpose(input_grids);
    t_grid.into_iter().map(|v| compress(v)).sum()
}

fn compress(input_vec: Vec<CentiMath>) ->  u64 {
    let mut v_iter = input_vec.iter().rev();
    let Some(CentiMath::Operator(o)) = v_iter.next() else {panic!("not an operator")};
    match o {
        CentiOperator::Mul => v_iter.fold(1, |acc, &n| {
            if let CentiMath::Number(num) = n {
                acc * num
            } else { panic!("not a number") }
        }),
        CentiOperator::Add => v_iter.fold(0, |acc, &n| {
            if let CentiMath::Number(num) = n {
                acc + num
            } else { panic!("not a number") }
        }),
    }
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &str = include_str!("test.txt");
    #[test]
    fn part1() {
        let out = crate::solve1(TEST_INPUT);
        assert_eq!(out, 4277556);
    }
}
