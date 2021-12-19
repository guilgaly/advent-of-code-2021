use common::lazy_static::lazy_static;
use common::regex::Regex;
use std::fmt::Write;

static INPUT_1: &str = include_str!("input_1");
static INPUT_2: &str = include_str!("input_2");

fn main() -> Result<(), String> {
    let dots = parse_dots(INPUT_1)?;
    let folds = parse_folds(INPUT_2)?;

    let part_1_result = part_1(&dots, &folds)?;
    println!("Part 1 result: {}", part_1_result);

    let part_2_result = part_2(&dots, &folds)?;
    println!("Part 2 result:\n{}", part_2_result);

    Ok(())
}

fn part_1(init_dots: &Vec<Dot>, folds: &Vec<Fold>) -> Result<usize, String> {
    let mut dots = init_dots.clone();
    let fold = folds.get(0).ok_or("Missing first fold")?;
    apply_fold(&mut dots, &fold);
    Ok(dots.len())
}

fn part_2(init_dots: &Vec<Dot>, folds: &Vec<Fold>) -> Result<String, String> {
    let mut dots = init_dots.clone();
    for fold in folds {
        apply_fold(&mut dots, fold);
    }

    let max_x = dots.iter().map(|d| d.x).max().ok_or("No dot found")?;
    let max_y = dots.iter().map(|d| d.y).max().ok_or("No dot found")?;

    let mut target_string = String::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if dots.contains(&Dot{x, y}) {
                write!(target_string, "#");
            } else {
                write!(target_string, " ");
            }
        }
        writeln!(target_string, "");
    }
    Ok(target_string)
}

fn apply_fold(dots: &mut Vec<Dot>, fold: &Fold) {
    match fold {
        Fold::X(v) => {
            for dot in dots.iter_mut() {
                if dot.x > *v {
                    (*dot).x = 2 * v - dot.x;
                }
            }
        }
        Fold::Y(v) => {
            for dot in dots.iter_mut() {
                if dot.y > *v {
                    (*dot).y = 2 * v - dot.y;
                }
            }
        }
    }
    dots.sort();
    dots.dedup();
}

fn parse_dots(input: &str) -> Result<Vec<Dot>, String> {
    fn parse_dot(line: &str) -> Option<Dot> {
        let mut split = line.split(",");
        let x = split.next()?.parse::<usize>().ok()?;
        let y = split.next()?.parse::<usize>().ok()?;
        Some(Dot { x, y })
    }
    input
        .lines()
        .map(|line| parse_dot(line).ok_or(format!("Failed to parse dot from line '{}'", line)))
        .collect()
}

fn parse_folds(input: &str) -> Result<Vec<Fold>, String> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"fold along ([xy])=([0-9]+)").unwrap();
    }
    input
        .lines()
        .map(|line| {
            REGEX
                .captures(line)
                .and_then(|cap| {
                    let c = cap.get(1)?.as_str();
                    let v = cap.get(2)?.as_str().parse::<usize>().ok()?;
                    if c == "x" {
                        Some(Fold::X(v))
                    } else {
                        Some(Fold::Y(v))
                    }
                })
                .ok_or(format!("Failed to parse fold from line '{}'", line))
        })
        .collect()
}

#[derive(Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct Dot {
    x: usize,
    y: usize,
}

enum Fold {
    X(usize),
    Y(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_dots() -> Vec<Dot> {
        parse_dots("6,10\n0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12\n4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0").unwrap()
    }

    fn test_folds() -> Vec<Fold> {
        parse_folds("fold along y=7\nfold along x=5").unwrap()
    }

    #[test]
    fn test_part_1() {
        let actual = part_1(&test_dots(), &test_folds()).unwrap();
        let expected = 17;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {}
}
