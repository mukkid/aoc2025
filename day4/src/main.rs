const INPUT_BYTES: &[u8] = include_bytes!("input.txt");

#[derive(Debug, Clone)]
struct Matrix {
    grid: Vec<u8>,
    rows: usize,
    cols: usize,
}

//(row, col)
type Dim = (isize, isize);

impl Matrix {
    fn from_bytes(input_bytes: &[u8]) -> Self {
        let cols = input_bytes.iter().position(|&c| c == b'\n').unwrap_or(0);
        let rows = input_bytes.iter().filter(|&c| *c == b'\n').count();
        let filtered_input: Vec<u8> =
            Vec::from_iter(input_bytes.iter().copied().filter(|&c| c != b'\n'));
        Self {
            grid: filtered_input,
            rows,
            cols,
        }
    }

    fn get_at(&self, index: Dim) -> Option<u8> {
        if index.0 < 0
            || index.1 < 0
            || index.0 >= self.rows as isize
            || index.1 >= self.cols as isize
        {
            return None;
        }
        Some(self.grid[index.0 as usize * self.rows + index.1 as usize])
    }
}

fn solve1(input_bytes: &[u8]) -> usize {
    let m = Matrix::from_bytes(input_bytes);
    let mut count = 0;
    for idx in 0..m.grid.len() {
        if m.grid[idx] != b'@' {
            continue;
        }
        let row = (idx / m.rows) as isize;
        let col = (idx % m.cols) as isize;
        let neighbors = [
            m.get_at((row - 1, col - 1)),
            m.get_at((row - 1, col)),
            m.get_at((row - 1, col + 1)),
            m.get_at((row, col - 1)),
            m.get_at((row, col + 1)),
            m.get_at((row + 1, col - 1)),
            m.get_at((row + 1, col)),
            m.get_at((row + 1, col + 1)),
        ];
        let rolls = neighbors
            .iter()
            .filter(|&c| matches!(*c, Some(b'@')))
            .count();
        if rolls < 4 {
            count += 1;
        }
    }
    count
}

fn solve2(input_bytes: &[u8]) -> usize {
    let mut m = Matrix::from_bytes(input_bytes);
    let mut count = 0;
    loop {
        let grid = m.grid.clone();
        for idx in 0..m.grid.len() {
            if m.grid[idx] != b'@' {
                continue;
            }
            let row = (idx / m.rows) as isize;
            let col = (idx % m.cols) as isize;
            let neighbors = [
                m.get_at((row - 1, col - 1)),
                m.get_at((row - 1, col)),
                m.get_at((row - 1, col + 1)),
                m.get_at((row, col - 1)),
                m.get_at((row, col + 1)),
                m.get_at((row + 1, col - 1)),
                m.get_at((row + 1, col)),
                m.get_at((row + 1, col + 1)),
            ];
            let rolls = neighbors
                .iter()
                .filter(|&c| matches!(*c, Some(b'@')))
                .count();
            if rolls < 4 {
                count += 1;
                m.grid[idx] = b'.';
            }
        }
        if m.grid == grid {
            break;
        }
    }
    count
}

fn main() {
    let out1 = solve1(INPUT_BYTES);
    let out2 = solve2(INPUT_BYTES);
    println!("{out1}");
    println!("{out2}");
}

#[cfg(test)]
mod test {
    const TEST_BYTES: &[u8] = include_bytes!("test.txt");
    #[test]
    fn part1() {
        let out = crate::solve1(TEST_BYTES);
        assert_eq!(out, 13);
    }

    #[test]
    fn part2() {
        let out = crate::solve2(TEST_BYTES);
        assert_eq!(out, 43)
    }
}
