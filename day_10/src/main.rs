use crate::Bracket::{Close, Open};
use crate::BracketType::{Angle, Curly, Round, Square};
use crate::ValidationResult::{Corrupted, Incomplete, Valid};
use common::itertools::Itertools;
use std::collections::{BinaryHeap, VecDeque};
use std::fmt::format;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    let lines = parse_input(INPUT)?;

    let part_1_result = part_1(&lines);
    println!("Part 1 result: {}", part_1_result);

    let part_2_result = part_2(&lines)?;
    println!("Part 2 result: {}", part_2_result);

    Ok(())
}

fn part_1<U>(lines: &[U]) -> usize
where
    U: AsRef<[Bracket]>,
{
    lines
        .iter()
        .map(|line| validate(line.as_ref()))
        .fold(0, |acc, res| match res {
            Valid => acc,
            Incomplete { .. } => acc,
            Corrupted { found, .. } => acc + illegal_bracket_value(&found),
        })
}

fn illegal_bracket_value(bracket: &BracketType) -> usize {
    match bracket {
        Round => 3,
        Square => 57,
        Curly => 1197,
        Angle => 25137,
    }
}

fn part_2<U>(lines: &[U]) -> Result<usize, String>
where
    U: AsRef<[Bracket]>,
{
    let completion_scores = lines
        .iter()
        .filter_map(|line| match validate(line.as_ref()) {
            Incomplete { missing } => Some(completion_value(&missing)),
            _ => None,
        })
        .sorted()
        .collect::<Vec<_>>();
    let winner_index = completion_scores.len() / 2;
    completion_scores
        .get(winner_index)
        .map(|res| *res)
        .ok_or(format!(
            "Wrong number of incomplete lines: {}",
            completion_scores.len()
        ))
}

fn completion_value(completion: &[BracketType]) -> usize {
    completion.iter().fold(0, |acc, next| {
        let next_value = match next {
            Round => 1,
            Square => 2,
            Curly => 3,
            Angle => 4,
        };
        acc * 5 + next_value
    })
}

fn parse_input(input: &str) -> Result<Vec<Vec<Bracket>>, String> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '(' => Ok(Open(Round)),
                    '[' => Ok(Open(Square)),
                    '{' => Ok(Open(Curly)),
                    '<' => Ok(Open(Angle)),
                    ')' => Ok(Close(Round)),
                    ']' => Ok(Close(Square)),
                    '}' => Ok(Close(Curly)),
                    '>' => Ok(Close(Angle)),
                    _ => Err(format!("Illegal character {}", c)),
                })
                .collect::<Result<Vec<Bracket>, String>>()
        })
        .collect::<Result<Vec<Vec<Bracket>>, String>>()
}

enum Bracket {
    Open(BracketType),
    Close(BracketType),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum BracketType {
    Round,
    Square,
    Curly,
    Angle,
}

fn validate(line: &[Bracket]) -> ValidationResult {
    let mut stack: Vec<BracketType> = Vec::new();

    for bracket in line {
        match bracket {
            Open(bracket_type) => stack.push(*bracket_type),
            Close(found) => match stack.pop() {
                None => return Corrupted { expected: None, found: *found },
                Some(expected) if expected != *found => {
                    return Corrupted { expected: Some(expected), found: *found };
                }
                _ => {}
            },
        }
    }

    if stack.is_empty() {
        Valid
    } else {
        stack.reverse();
        Incomplete { missing: stack }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum ValidationResult {
    Valid,
    Incomplete {
        missing: Vec<BracketType>,
    },
    Corrupted {
        expected: Option<BracketType>,
        found: BracketType,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part_1() -> Result<(), String> {
        let lines = parse_input(TEST_INPUT)?;
        let actual = part_1(&lines);
        let expected: usize = 26397;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), String> {
        let lines = parse_input(TEST_INPUT)?;
        let actual = part_2(&lines)?;
        let expected: usize = 288957;
        assert_eq!(expected, actual);
        Ok(())
    }
}
