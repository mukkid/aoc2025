const INPUT_STR: &str = include_str!("input.txt");

fn main() {
    let out1 = solve1(INPUT_STR);
    let out2 = solve2(INPUT_STR);
    println!("{out1}, {out2}");
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vertex {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Polygon {
    vertices: Vec<Vertex>
}

impl Polygon {
    fn contains(&self, vertex: Vertex) -> bool {
        if self.vertices.contains(&vertex) {
            return true
        }
        // loop the list of vertices back to form a closed loop
        let first_wrap = [*self.vertices.last().unwrap(), *self.vertices.first().unwrap()];
        let mut v_iter = self.vertices.windows(2).chain(std::iter::once(first_wrap.as_slice()));
        let mut crossings = 0;
        while let Some([v1, v2, ..]) = v_iter.next() {
            if v1.x != v2.x {
                continue;
            }
            if v1.x > vertex.x && v1.y.min(v2.y) <= vertex.y && vertex.y <= v1.y.max(v2.y) {
                crossings += 1;
            }
        }
        crossings % 2 == 1
    }
}

fn solve2(input_str: &str) -> i32 {
    let p: Polygon = Polygon { vertices: input_str.lines().map(|l| {
        if let Some((n1, n2)) = l.split_once(",") {
            let (x, y) = (n1.parse::<i32>().unwrap(), n2.parse::<i32>().unwrap());
            Vertex {x, y}
        }
        else {
            panic!("Not a number");
        }
    }).collect()};
    let mut largest_area = 0;
    for v1 in p.vertices[..p.vertices.len()].iter() {
        for v2 in p.vertices[1..].iter() {
            let corner1 = Vertex {x: v1.x, y: v2.y};
            let corner2 = Vertex {x: v2.x, y: v1.y};
            if p.contains(corner1) && p.contains(corner2) {
                let dy = (v2.y - v1.y).abs() + 1;
                let dx = (v2.x - v1.x).abs() + 1;
                let area = dx * dy;
                if area > largest_area {
                    largest_area = area;
                }
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
    #[test]
    fn part2() {
        let out = crate::solve2(TEST_STR);
        assert_eq!(out, 24);
    }
}
