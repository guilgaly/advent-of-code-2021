use common::itertools::Itertools;
use common::lazy_static::lazy_static;
use common::regex::Regex;

static INPUT: &str = include_str!("input");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

fn main() -> Result<(), String> {
    let vent_lines = parse_input(INPUT)?;
    println!(
        "Part 1 result: {}",
        count_overlap_points(&vent_lines, horiz_and_vert_points)
    );
    println!(
        "Part 2 result: {}",
        count_overlap_points(&vent_lines, horiz_vert_and_45_points)
    );
    Ok(())
}

fn count_overlap_points(vent_lines: &[Line], f: fn(&Line) -> Vec<Point>) -> usize {
    vent_lines
        .iter()
        .flat_map(f)
        .map(|point| point)
        .sorted()
        .dedup_with_count()
        .filter(|(occurrences, _)| *occurrences > 1)
        .count()
}

fn horiz_and_vert_points(line: &Line) -> Vec<Point> {
    if line.x1 == line.x2 {
        range(line.y1, line.y2)
            .map(|y| Point { x: line.x1, y })
            .collect::<Vec<_>>()
    } else if line.y1 == line.y2 {
        range(line.x1, line.x2)
            .map(|x| Point { x, y: line.y1 })
            .collect::<Vec<_>>()
    } else {
        vec![]
    }
}

fn horiz_vert_and_45_points(line: &Line) -> Vec<Point> {
    if line.x1 == line.x2 {
        range(line.y1, line.y2)
            .map(|y| Point { x: line.x1, y })
            .collect::<Vec<_>>()
    } else if line.y1 == line.y2 {
        range(line.x1, line.x2)
            .map(|x| Point { x, y: line.y1 })
            .collect::<Vec<_>>()
    } else {
        range(line.x1, line.x2)
            .zip(range(line.y1, line.y2))
            .map(|(x, y)| Point { x, y })
            .collect::<Vec<_>>()
    }
}

fn range(v1: usize, v2: usize) -> Box<dyn Iterator<Item = usize>> {
    if v1 < v2 {
        Box::new(v1..=v2)
    } else {
        Box::new((v2..=v1).rev())
    }
}

fn parse_input(input: &str) -> Result<Vec<Line>, String> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    }
    input
        .lines()
        .map(|line| {
            REGEX
                .captures(line)
                .and_then(|cap| {
                    let x1 = cap.get(1)?.as_str().parse::<usize>().ok()?;
                    let y1 = cap.get(2)?.as_str().parse::<usize>().ok()?;
                    let x2 = cap.get(3)?.as_str().parse::<usize>().ok()?;
                    let y2 = cap.get(4)?.as_str().parse::<usize>().ok()?;
                    Some(Line { x1, y1, x2, y2 })
                })
                .ok_or(format!("Failed to parse line {}", line))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_LINES: [Line; 10] = [
        Line {
            x1: 0,
            y1: 9,
            x2: 5,
            y2: 9,
        },
        Line {
            x1: 8,
            y1: 0,
            x2: 0,
            y2: 8,
        },
        Line {
            x1: 9,
            y1: 4,
            x2: 3,
            y2: 4,
        },
        Line {
            x1: 2,
            y1: 2,
            x2: 2,
            y2: 1,
        },
        Line {
            x1: 7,
            y1: 0,
            x2: 7,
            y2: 4,
        },
        Line {
            x1: 6,
            y1: 4,
            x2: 2,
            y2: 0,
        },
        Line {
            x1: 0,
            y1: 9,
            x2: 2,
            y2: 9,
        },
        Line {
            x1: 3,
            y1: 4,
            x2: 1,
            y2: 4,
        },
        Line {
            x1: 0,
            y1: 0,
            x2: 8,
            y2: 8,
        },
        Line {
            x1: 5,
            y1: 5,
            x2: 8,
            y2: 2,
        },
    ];

    #[test]
    fn test_part_1() {
        let actual = count_overlap_points(&TEST_LINES, horiz_and_vert_points);
        let expected = 5;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let actual = count_overlap_points(&TEST_LINES, horiz_vert_and_45_points);
        let expected = 12;
        assert_eq!(expected, actual);
    }
}
