use crate::signals::{Segment, SignalPattern};
use common::lazy_static::lazy_static;
use common::regex::Regex;
use std::collections::HashSet;
use std::convert::TryInto;

mod signals;

static INPUT: &str = include_str!("input");

/// Index = crab position, value = number of crabs with that position.
/// E.g. if `crabs[2] == 5`, there are 5 crabs at position 2.
type Crabs = [usize];

fn main() -> Result<(), String> {
    // let initial_positions = parse_input(INPUT)?;

    let lines = parse_input(&INPUT)?;

    let part_1_result = part_1(&lines);
    println!("Part 1 result: {}", part_1_result);
    //
    // let part_2_result = lowest_fuel_expenditure(&initial_positions, actual_fuel_cost)
    //     .ok_or("Result 1 not found")?;
    // println!("Part 2 result: {}", part_2_result);

    Ok(())
}

fn part_1(lines: &[Line]) -> usize {
    lines.iter().fold(0, |acc, line| {
        acc + line
            .digits
            .iter()
            .filter(|digit| {
                let num_seg = digit.0.len();
                num_seg == 2 || num_seg == 4 || num_seg == 3 || num_seg == 7
            })
            .count()
    })
}

fn part_2(lines: &[Line]) -> Result<usize, String> {
    let mut acc: usize = 0;

    for line in lines {
        let digit_1 = line.patterns.iter().find(|p| p.0.len() == 1).ok_or("Digit 1 not found")?;
        let digit_4 = line.patterns.iter().find(|p| p.0.len() == 4).ok_or("Digit 4 not found")?;
        let digit_7 = line.patterns.iter().find(|p| p.0.len() == 3).ok_or("Digit 7 not found")?;
        let digit_8 = line.patterns.iter().find(|p| p.0.len() == 7).ok_or("Digit 8 not found")?;
        let digits_6_9 = line.patterns.iter().filter(|p| p.0.len() == 6).collect::<Vec<_>>();
        let digits

    }

    Ok(acc)
}



fn parse_input(input: &str) -> Result<Vec<Line>, String> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<Line, String> {
    let mut parts = line.split(" | ");
    let patterns = parts
        .next()
        .ok_or(format!("Missing first part in line {}", line))?
        .split(' ')
        .map(|s| s.parse::<SignalPattern>())
        .collect::<Result<Vec<SignalPattern>, String>>()?;
    let digits: [SignalPattern; 4] = parts
        .next()
        .ok_or(format!("Missing second part in line {}", line))?
        .split(' ')
        .map(|s| s.parse::<SignalPattern>())
        .collect::<Result<Vec<SignalPattern>, String>>()?
        .try_into()
        .map_err(|_| "Wrong number of digits")?;
    Ok(Line { patterns, digits })
}

struct Line {
    patterns: Vec<SignalPattern>,
    digits: [SignalPattern; 4],
}

#[cfg(test)]
mod tests {
    use super::*;

    // static TEST_CRABS: [usize; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_part_1() {}

    #[test]
    fn test_part_2() {}
}
