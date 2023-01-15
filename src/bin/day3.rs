use anyhow::Result;
use std::collections::HashMap;
use std::{num::ParseIntError, str::FromStr};

struct Power {
    gamma: String,
    epsilon: String,
}

impl Power {
    fn answer(&self) {
        let g = u32::from_str_radix(&self.gamma, 2).unwrap();
        let e = u32::from_str_radix(&self.epsilon, 2).unwrap();
        println!("{}", g * e);
    }
}

fn main() -> Result<()> {
    let mut map = HashMap::new();
    let total_lines = include_str!("./day3_input.txt")
        .lines()
        .map(|line| {
            for (i, c) in line.chars().enumerate() {
                let cur = map.get(&i).unwrap_or(&0);
                let num = c.to_digit(10).unwrap();
                let new = cur + num;
                map.insert(i, new);
            }
            1
        })
        .sum::<u32>();

    let mut power = Power {
        gamma: String::new(),
        epsilon: String::new(),
    };

    // itering through a hash map does not garuntee order!!
    for i in 0..12 {
        let v = map.get(&i).unwrap();
        if v > &500 {
            power.gamma.push('1');
            power.epsilon.push('0');
        } else {
            power.gamma.push('0');
            power.epsilon.push('1');
        }
    }

    power.answer();

    Ok(())
}
