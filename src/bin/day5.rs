use anyhow::{Context, Result};
use aoc::read_one_at_a_time;
use std::{collections, str::FromStr};

struct Line {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

fn parse_set(s: &str) -> Result<(i32, i32), anyhow::Error> {
    let (x, y) = s.split_once(",").context("no comma in set found.")?;
    let x = x.parse()?;
    let y = y.parse()?;

    Ok((x, y))
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (set1, set2) = s.split_once(" -> ").context("invalid split")?;
        let set1 = parse_set(set1)?;
        let set2 = parse_set(set2)?;

        Ok(Line {
            x1: set1.0,
            x2: set2.0,
            y1: set1.1,
            y2: set2.1,
        })
    }
}

impl Line {
    fn get_line_coords(&self) -> Vec<String> {
        let mut res = Vec::new();
        if self.x1 != self.x2 && self.y1 == self.y2 {
            let dif = self.x2 - self.x1;
            if dif > 0 {
                for i in 0..dif + 1 {
                    res.push(format!("{},{}", self.x1 + i, self.y1));
                }
            } else {
                for i in dif..1 {
                    res.push(format!("{},{}", self.x1 + i, self.y1));
                }
            }
        } else if self.y1 != self.y2 && self.x1 == self.x2 {
            let dif = self.y2 - self.y1;
            if dif > 0 {
                for i in 0..dif + 1 {
                    res.push(format!("{},{}", self.x1, self.y1 + i));
                }
            } else {
                for i in dif..1 {
                    res.push(format!("{},{}", self.x1, self.y1 + i));
                }
            }
        }
        let dify = self.y1 - self.y2;
        let difx = self.x1 - self.x2;

        if dify.abs() == difx.abs() {
            if difx.signum() < 0 && dify.signum() < 0 {
                for i in dify..1 {
                    res.push(format!("{},{}", self.x1 - i, self.y1 - i));
                }
            } else if difx.signum() < 0 {
                for i in difx..1 {
                    res.push(format!("{},{}", self.x1 - i, self.y1 + i));
                }
            } else if dify.signum() < 0 {
                for i in dify..1 {
                    res.push(format!("{},{}", self.x1 + i, self.y1 - i));
                }
            } else {
                for i in 0..dify.abs() + 1 {
                    res.push(format!("{},{}", self.x1 - i, self.y1 - i))
                }
            }
        }
        res
    }
}

fn greater_than_two(num: &i32) -> Option<i32> {
    if num >= &2 {
        return Some(*num);
    }
    None
}

fn main() -> Result<()> {
    let lines = read_one_at_a_time::<Line>("./data/5.input")?;
    let mut map = collections::HashMap::new();

    lines.iter().for_each(|line| {
        for cord in line.get_line_coords() {
            let _ = *map.entry(cord).and_modify(|c| *c += 1).or_insert(1);
        }
    });

    let res: usize = map
        .iter()
        .filter_map(|(_, v)| greater_than_two(&v))
        .collect::<Vec<i32>>()
        .len();

    println!("Part One: {res}");

    Ok(())
}
