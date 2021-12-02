use std::convert::TryFrom;
use std::error::Error;
use std::num::ParseIntError;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let depths = parse_input(INPUT)?;
    let depths_increases = count_increases(&depths);
    println!("Part 1 result: {}", depths_increases);
    let amortized_depths_increases = count_amortized_increases(&depths);
    println!("Part 2 result: {}", amortized_depths_increases);
    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|line| line.parse::<i32>()).collect()
}

fn count_increases(input: &Vec<i32>) -> i32 {
    input
        .windows(2)
        .flat_map(<&[i32; 2]>::try_from)
        .fold(0, |acc, [x, y]| if y > x { acc + 1 } else { acc })
}

fn count_amortized_increases(input: &Vec<i32>) -> i32 {
    let window_sums: Vec<i32> = input.windows(3).map(|x| x.iter().sum()).collect();
    count_increases(&window_sums)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_part_1() {}
}
