use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("./day1_input.txt")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut res = 0;

    for v in input.windows(2) {
        if v[0] < v[1] {
            res += 1;
        }
    }

    let mut prev = 0;
    let mut next = 0;
    let mut part_two_res = 0;
    for i in input.windows(3) {
        next = i.iter().sum();
        if next > prev {
            part_two_res += 1;
        }
        prev = next.clone();
    }

    println!("Part 1: {res}");
    // Subtract one b/c the first conparsion will always pass falsely.
    println!("Part 2: {}", part_two_res - 1);
    Ok(())
}
