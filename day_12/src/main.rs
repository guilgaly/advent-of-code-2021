use crate::Cave::{End, Large, Small, Start};
use common::itertools::Itertools;
use common::lazy_static::lazy_static;
use common::regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    let paths = parse_input(INPUT)?;

    let part_1_result = part_1(&paths);
    println!("Part 1 result: {}", part_1_result);

    let part_2_result = part_2(&paths);
    println!("Part 2 result: {}", part_2_result);

    Ok(())
}

fn parse_input(input: &str) -> Result<HashMap<Cave, Vec<Cave>>, String> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"([a-zA-Z]+)-([a-zA-Z]+)").unwrap();
    }

    let links: Vec<(Cave, Cave)> = input
        .lines()
        .flat_map(|line| {
            let parsed = REGEX.captures(line).and_then(|cap| {
                let orig = cap.get(1)?.as_str().parse::<Cave>().ok()?;
                let dest = cap.get(2)?.as_str().parse::<Cave>().ok()?;
                Some((orig, dest))
            });
            match parsed {
                None => Vec::new(),
                Some((Start, b)) => vec![(Start, b)],
                Some((a, Start)) => vec![(Start, a)],
                Some((End, b)) => vec![(b, End)],
                Some((a, End)) => vec![(a, End)],
                Some((a, b)) => vec![(a.clone(), b.clone()), (b, a)],
            }
        })
        .collect::<Vec<(Cave, Cave)>>();
    Ok(links.into_iter().into_group_map())
}

fn part_1(paths: &HashMap<Cave, Vec<Cave>>) -> usize {
    fn next(paths: &HashMap<Cave, Vec<Cave>>, current: &Cave, path: Vec<&Cave>) -> usize {
        match current {
            End => 1,
            Small(_) if path.contains(&current) => 0,
            _ => paths
                .get(current)
                .unwrap_or(&Vec::new())
                .iter()
                .fold(0, |acc, dest| {
                    let mut new_path = path.clone();
                    new_path.push(current);
                    acc + next(paths, dest, new_path)
                }),
        }
    }

    next(paths, &Start, Vec::new())
}

fn part_2(paths: &HashMap<Cave, Vec<Cave>>) -> usize {
    fn next(paths: &HashMap<Cave, Vec<Cave>>, current: &Cave, path: Vec<&Cave>) -> usize {
        // println!("path: {:?}, current: {:?}", path, current);
        match current {
            End => 1,
            Small(_) if path.contains(&current) && small_visited_twice(&path) => 0,
            _ => paths
                .get(current)
                .unwrap_or(&Vec::new())
                .iter()
                .fold(0, |acc, dest| {
                    let mut new_path = path.clone();
                    new_path.push(current);
                    acc + next(paths, dest, new_path)
                }),
        }
    }

    fn small_visited_twice(path: &Vec<&Cave>) -> bool {
        let mut remaining = path.clone();
        while let Some(curr) = remaining.pop() {
            match curr {
                Small(_) if remaining.contains(&curr) => {
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    next(paths, &Start, Vec::new())
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone)]
enum Cave {
    Start,
    End,
    Small(String),
    Large(String),
}

impl FromStr for Cave {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "start" {
            Ok(Start)
        } else if s == "end" {
            Ok(End)
        } else if s.to_lowercase() == s {
            Ok(Small(s.to_owned()))
        } else if s.to_uppercase() == s {
            Ok(Large(s.to_owned()))
        } else {
            Err(format!("{} is not a valid cave name", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SMALL_INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    static LARGER_INPUT: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    static LARGE_INPUT: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_part_1() {
        fn test(input: &str, expected: usize) {
            let paths = parse_input(input).unwrap();
            let actual = part_1(&paths);
            assert_eq!(expected, actual);
        }

        test(SMALL_INPUT, 10);
        test(LARGER_INPUT, 19);
        test(LARGE_INPUT, 226);
    }

    #[test]
    fn test_part_2() {
        fn test(input: &str, expected: usize) {
            let paths = parse_input(input).unwrap();
            let actual = part_2(&paths);
            assert_eq!(expected, actual);
        }

        test(SMALL_INPUT, 36);
        test(LARGER_INPUT, 103);
        test(LARGE_INPUT, 3509);
    }
}
