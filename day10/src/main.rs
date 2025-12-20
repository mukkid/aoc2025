use std::fmt;
use regex::Regex;

const INPUT_STR: &str = include_str!("input.txt");

fn main() {
    let out1 = solve1(INPUT_STR);
    println!("{out1}");
}

struct LightMachine {
    len: usize,
    target_light: u16,
    current_light: u16,
    buttons: Vec<u16>
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
            for idx in 0..=self.len+1 {
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

impl fmt::Debug for LightMachine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "len: {}\n", self.len);
        let _ = write!(f, "target_light: {:0len$b}\n", self.target_light.reverse_bits() >> 16-self.len, len = self.len);
        let _ = write!(f, "current_light: {:0len$b}\n", self.current_light.reverse_bits() >> 16-self.len, len = self.len);
        let _ = write!(f, "buttons: {{\n");
        for b in &self.buttons {
            let _ = write!(f, "\t{:0len$b}\n", b.reverse_bits() >> 16-self.len, len = self.len);
        }
        write!(f, "}}")
    }
}

fn solve1(input_str: &str) -> usize {
    let re = Regex::new(r"\[([^\]]+)\] (\([^{]+) \{([^}]+)\}").unwrap();
    let mut lms = vec![];
    for (_, [lights, buttons, _]) in re.captures_iter(input_str).map(|c| c.extract()) {
        // reversed because the 0th digit is the leftmost one
        let l = lights.chars().rev().fold(0b0, |acc, c| {
            if c == '.' {
                acc << 1
            } else {
                (acc | 0b1) << 1
            }
        }) >> 1;

        let b: Vec<u16> = buttons.split_whitespace().map(|b| {
            b[1..b.len()-1].split(',').fold(0_u16, |acc, bm| {
                acc | 1 << bm.parse::<u8>().unwrap()
            })
                
        }).collect();
        let lm = LightMachine { len: lights.len(), target_light: l, current_light: 0, buttons: b };
        lms.push(lm);
    }
    lms.iter().map(|lm| lm.press_buttons()).sum::<usize>()
}

#[cfg(test)]
mod test {
    const TEST_STR: &str = include_str!("test.txt");

    #[test]
    fn part1() {
        let out = crate::solve1(TEST_STR);
        assert_eq!(out, 7);
    }
}
