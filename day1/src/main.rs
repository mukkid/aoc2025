use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("test.txt").unwrap();
    let f = BufReader::new(f);
    let mut dial = 50;
    let mut zero_count = 0;
    for line in f.lines() {
        if let Ok(l) = line {
            let direction: i8 = ((l.bytes().nth(0).unwrap() & 2).wrapping_sub(1)) as i8;
            // R = b0101_0010
            // L = b0100_1100
            let magnitude = &l[1..].parse::<i32>().unwrap();
            let v = unsafe { magnitude.unchecked_mul(direction.into()) };
            dial += v % 100;
            dial %= 100;
            if dial == 0 {
                zero_count += 1;
            }
        };
    }
    println!("{zero_count}");
}
