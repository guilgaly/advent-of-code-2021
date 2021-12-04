mod command;
mod part_1;
mod part_2;

use crate::command::Command;
use crate::part_1::part_1_result;
use crate::part_2::part_2_result;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    let commands = parse_input(INPUT)?;
    println!("Part 1 result: {}", part_1_result(&commands));
    println!("Part 2 result: {}", part_2_result(&commands));
    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<Command>, String> {
    let commands = input
        .lines()
        .map(|line| line.parse::<Command>())
        .collect::<Result<Vec<Command>, String>>()?;
    Ok(commands)
}
