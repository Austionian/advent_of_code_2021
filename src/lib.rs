use anyhow::Result;
use std::str::FromStr;

/// Function that parse every line in a txt file to a
/// given generic.
pub fn read_one_at_a_time<T>(path: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

pub fn split_on_comma<T>(path: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .split(',')
        .filter_map(|v| v.trim().parse().ok())
        .collect())
}
