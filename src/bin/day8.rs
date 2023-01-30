use anyhow::{Context, Result};
use aoc::read_one_at_a_time;
use std::collections::HashMap;
use std::str::FromStr;

struct Digs {
    input: Vec<String>,
    output: Vec<String>,
}

const SUMS: [u32; 10] = [42, 17, 34, 39, 30, 37, 41, 25, 49, 45];

impl FromStr for Digs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, output) = s.split_once(" | ").context("no pipe found.")?;
        Ok(Digs {
            input: input.split_whitespace().map(|s| s.to_string()).collect(),
            output: output.split_whitespace().map(|s| s.to_string()).collect(),
        })
    }
}

const LENS: [u32; 4] = [2, 4, 3, 7];

fn find_easy() -> Result<u32> {
    Ok(read_one_at_a_time::<Digs>("./data/8.input")?
        .iter()
        .flat_map(|v| {
            v.output
                .iter()
                .map(|i| {
                    let j = i.len() as u32;
                    if LENS.contains(&j) {
                        1
                    } else {
                        0
                    }
                })
                .collect::<Vec<u32>>()
        })
        .sum())
}

fn find_hard() -> Result<u32> {
    let values = read_one_at_a_time::<Digs>("./data/8.input")?;

    Ok(values
        .iter()
        .map(|v| {
            let mut map = HashMap::new();
            for i in v.input.iter() {
                i.chars().for_each(|c| {
                    map.entry(c).and_modify(|cc| *cc += 1).or_insert(1);
                });
            }

            let mut nums = vec![];

            v.output.iter().for_each(|j| {
                nums.push(
                    j.chars()
                        .map(|c| map.get(&c).expect("value not in map").clone())
                        .sum::<u32>(),
                )
            });

            let value = nums
                .iter()
                .map(|num| SUMS.iter().position(|&i| i == *num).unwrap().to_string())
                .collect::<String>();

            value.parse::<u32>().unwrap()
        })
        .sum::<u32>())
}

fn main() -> Result<()> {
    println!("Part One: {}", find_easy()?);
    println!("Part Two: {}", find_hard()?);
    Ok(())
}
