use regex::Regex;
use std::fmt;

const INPUT_STR: &str = include_str!("input.txt");

fn main() {
    let (out1, out2) = solve1(INPUT_STR);
    println!("{out1}");
    println!("{out2}");
}

#[derive(Debug)]
struct LightMachine {
    len: usize,
    target_light: u16,
    current_light: u16,
    buttons: Vec<u16>,
}

#[derive(Debug)]
struct JoltMachine {
    joltages: Vec<usize>,
    buttons: Vec<Vec<usize>>
}

// definitions:
// Button configurations is 0..2^n : n is number of buttons
// example:
// 0b 00010 -> press the second button
// 0b 00101 -> press the first and third button

impl LightMachine {
    fn press_buttons(&self) -> usize {
        let limit = 2_i32.pow(self.buttons.len() as u32);
        let mut fewest_presses = self.buttons.len();
        for press in 0..limit {
            let mut button_press = 0;
            for idx in 0..=self.len + 1 {
                if press >> idx & 0x1 == 0x1 {
                    button_press ^= self.buttons[idx];
                }
            }
            if button_press == self.target_light {
                let ones = press.count_ones() as usize;
                if ones <= fewest_presses {
                    fewest_presses = ones;
                }
            }
        }
        fewest_presses
    }
}

fn solve1(input_str: &str) -> (usize, usize) {
    let re = Regex::new(r"\[([^\]]+)\] (\([^{]+) \{([^}]+)\}").unwrap();
    let mut lms = vec![];
    let mut jms = vec![];
    for (_, [lights, buttons, joltages]) in re.captures_iter(input_str).map(|c| c.extract()) {
        // reversed because the 0th digit is the leftmost one
        let l = lights.chars().rev().fold(
            0b0,
            |acc, c| {
                if c == '.' { acc << 1 } else { (acc | 0b1) << 1 }
            },
        ) >> 1;

        let jb: Vec<Vec<usize>> = buttons
            .split_whitespace()
            .map(|b| b[1..b.len() - 1].split(",").map(|i| i.parse().unwrap()).collect())
            .collect();

        let b: Vec<u16> = buttons
            .split_whitespace()
            .map(|b| {
                b[1..b.len() - 1]
                    .split(',')
                    .fold(0_u16, |acc, bm| acc | 1 << bm.parse::<u8>().unwrap())
            })
            .collect();

        let j: Vec<usize> = joltages.split(",").map(|c| c.parse().unwrap()).collect();
        let lm = LightMachine {
            len: lights.len(),
            target_light: l,
            current_light: 0,
            buttons: b,
        };

        let jm = JoltMachine {
            joltages: j,
            buttons: jb,
        };
        lms.push(lm);
        jms.push(jm);
    }
    let lights = lms.iter().map(|lm| lm.press_buttons()).sum::<usize>();
    dbg!(jms);
    (lights, 0)
}

#[cfg(test)]
mod test {
    const TEST_STR: &str = include_str!("test.txt");

    #[test]
    fn part1() {
        let (out1, _) = crate::solve1(TEST_STR);
        assert_eq!(out1, 7);
    }

    #[test]
    fn part2() {
        let (_, out2) = crate::solve1(TEST_STR);
        assert_eq!(out2, 33);
    }
}
