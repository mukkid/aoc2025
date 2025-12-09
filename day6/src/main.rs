use ndarray::Array2;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Copy, Clone)]
enum CentiMath<'a> {
    Number(u32),
    Operator(&'a str)
}

fn transpose<'a>(matrix: Vec<Vec<CentiMath<'a>>>) -> Vec<Vec<CentiMath<'a>>> {
    (0..matrix[0].len())
        .map(|col| matrix.iter().map(|row| row[col]).collect())
        .collect()
}

fn solve1(input_str: &str) -> u32 {
    let input_grids: Vec<Vec<CentiMath>> = input_str.lines().map(|s| {
        s.split_whitespace().map(|sp|
        match sp.parse::<u32>() {
            Ok(n) => CentiMath::Number(n),
            Err(_) => CentiMath::Operator(sp),
        }).collect()
    }).collect();
    let t_grid = transpose(input_grids);
    dbg!(t_grid);
    0
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
