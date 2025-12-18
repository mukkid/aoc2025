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
            return true;
        }
        
        let n = self.vertices.len();
        for i in 0..n {
            let j = (i + 1) % n;
            let vi = self.vertices[i];
            let vj = self.vertices[j];
            
            if vi.x == vj.x && vi.x == vertex.x && 
               vertex.y >= vi.y.min(vj.y) && vertex.y <= vi.y.max(vj.y) {
                return true;
            }
            if vi.y == vj.y && vi.y == vertex.y && 
               vertex.x >= vi.x.min(vj.x) && vertex.x <= vi.x.max(vj.x) {
                return true;
            }
        }
        
        let mut inside = false;
        let mut j = n - 1;
        for i in 0..n {
            let vi = self.vertices[i];
            let vj = self.vertices[j];
            
            if ((vi.y > vertex.y) != (vj.y > vertex.y)) &&
               (vertex.x < (vj.x - vi.x) * (vertex.y - vi.y) / (vj.y - vi.y) + vi.x) {
                inside = !inside;
            }
            j = i;
        }
        inside
    }
}

fn compress_coordinates(vertices: &[Vertex]) -> (Vec<i32>, Vec<i32>, Vec<Vertex>) {
    let mut xs: Vec<i32> = vertices.iter().map(|v| v.x).collect();
    let mut ys: Vec<i32> = vertices.iter().map(|v| v.y).collect();
    xs.sort();
    xs.dedup();
    ys.sort();
    ys.dedup();
    
    let compressed_vertices: Vec<Vertex> = vertices.iter().map(|v| {
        let cx = xs.binary_search(&v.x).unwrap() as i32;
        let cy = ys.binary_search(&v.y).unwrap() as i32;
        Vertex { x: cx, y: cy }
    }).collect();
    
    (xs, ys, compressed_vertices)
}

fn solve2(input_str: &str) -> i32 {
    let vertices: Vec<Vertex> = input_str.lines().map(|l| {
        if let Some((n1, n2)) = l.split_once(",") {
            let (x, y) = (n1.parse::<i32>().unwrap(), n2.parse::<i32>().unwrap());
            Vertex {x, y}
        }
        else {
            panic!("Not a number");
        }
    }).collect();
    
    let (x_map, y_map, compressed_vertices) = compress_coordinates(&vertices);
    let p = Polygon { vertices: compressed_vertices };
    let mut largest_area = 0;
    
    for v1 in &p.vertices {
        'outer: for v2 in &p.vertices {
            if v1 == v2 { continue; }
            let (min_x, max_x) = (v1.x.min(v2.x), v1.x.max(v2.x));
            let (min_y, max_y) = (v1.y.min(v2.y), v1.y.max(v2.y));
            
            for x in min_x..=max_x {
                if !p.contains(Vertex { x, y: min_y }) || !p.contains(Vertex { x, y: max_y }) {
                    continue 'outer;
                }
            }
            for y in min_y+1..max_y {
                if !p.contains(Vertex { x: min_x, y }) || !p.contains(Vertex { x: max_x, y }) {
                    continue 'outer;
                }
            }
            
            let actual_area = (x_map[max_x as usize] - x_map[min_x as usize] + 1) * 
                             (y_map[max_y as usize] - y_map[min_y as usize] + 1);
            largest_area = largest_area.max(actual_area);
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
