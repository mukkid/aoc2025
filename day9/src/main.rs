const INPUT_STR: &str = include_str!("input.txt");

fn main() {
    let out1 = solve1(INPUT_STR);
    println!("{out1}");
}

fn solve1(input_str: &str) -> i64 {
    let coords: Vec<_> = input_str.lines().map(|l| {
        if let Some((n1, n2)) = l.split_once(",") {
            (n1.parse::<i64>().unwrap(), n2.parse::<i64>().unwrap())
        } else { panic!("failed to parse numbers") }
    })
    .collect();
    let mut largest_area = 0;
    for (idx1, &c1) in coords[..coords.len()-1].iter().enumerate() {
        for (_, &c2) in coords[idx1+1..].iter().enumerate() {
            let dy = (c2.1 - c1.1).abs() + 1;
            let dx = (c2.0 - c1.0).abs() + 1;
            let area = dx * dy;
            if area > largest_area {
                largest_area = area;
            }
        }
    }
    largest_area
}

#[cfg(test)]
mod test {
    const TEST_STR: &str = include_str!("test.txt");

    #[test]
    fn part1() {
        let out = crate::solve1(TEST_STR);
        assert_eq!(out, 50);
    }
}
