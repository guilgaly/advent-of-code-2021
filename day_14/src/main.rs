use common::itertools::Itertools;
use common::lazy_static::lazy_static;
use common::regex::Regex;
use std::collections::HashMap;
use std::fmt::Write;
use std::iter;

static INPUT_1: &str = include_str!("input_1");
static INPUT_2: &str = include_str!("input_2");

fn main() -> Result<(), String> {
    let polymer_template = parse_polymer_template(INPUT_1)?;
    let insertion_rules = parse_insertion_rules(INPUT_2)?;

    let part_1_result = polymerize(&polymer_template, &insertion_rules, 10)?;
    println!("Part 1 result: {}", part_1_result);

    // let part_2_result = polymerize(&polymer_template, &insertion_rules, 40)?;
    // println!("Part 2 result: {}", part_2_result);

    Ok(())
}

fn polymerize(
    polymer_template: &[Element],
    insertion_rules: &InsertionRules,
    steps: usize,
) -> Result<usize, String> {
    let mut polymer: Vec<Element> = polymer_template.iter().map(|e| *e).collect();

    for _ in 0..steps {
        let mut new_polymer = Vec::new();

        for i in 0..(polymer.len() - 1) {
            let p1 = polymer[i];
            new_polymer.push(p1);
            let p2 = polymer[i + 1];
            for i in insertion_rules.get(&(p1, p2)) {
                new_polymer.push(*i);
            }
        }
        new_polymer.push(polymer[polymer.len() - 1]);
        polymer = new_polymer;
    }

    polymer.sort();
    let counts_map = polymer.iter().counts();
    let mut counts = counts_map.values().sorted();
    let smallest = counts.next().ok_or("Smallest count not found")?;
    let largest = counts.last().ok_or("Largest count not found")?;
    println!("smallest={}", smallest);
    println!("largest={}", largest);

    Ok(largest - smallest)
}

fn parse_polymer_template(input: &str) -> Result<Vec<Element>, String> {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() && c.is_uppercase() {
                Ok(Element(c))
            } else {
                Err(format!("{} is not a valid element", c))
            }
        })
        .collect()
}

fn parse_insertion_rules(input: &str) -> Result<InsertionRules, String> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"([A-Z])([A-Z]) -> ([A-Z])").unwrap();
    }
    input
        .lines()
        .map(|line| {
            REGEX
                .captures(line)
                .and_then(|cap| {
                    let p1 = cap.get(1)?.as_str().chars().next()?;
                    let p2 = cap.get(2)?.as_str().chars().next()?;
                    let pair = (Element(p1), Element(p2));
                    let i = cap.get(3)?.as_str().chars().next()?;
                    let insertion = Element(i);
                    Some((pair, insertion))
                })
                .ok_or(format!(
                    "Failed to parse insertion rule from line '{}'",
                    line
                ))
        })
        .collect::<Result<InsertionRules, String>>()
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Element(char);
type Pair = (Element, Element);
type InsertionRules = HashMap<Pair, Element>;

#[cfg(test)]
mod tests {
    use super::*;

    static POLYMER_TEMPLATE: [Element; 4] =
        [Element('N'), Element('N'), Element('C'), Element('B')];

    fn insertion_rules() -> InsertionRules {
        parse_insertion_rules(
            "CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C",
        )
        .unwrap()
    }

    #[test]
    fn test_part_1() {
        let actual = polymerize(&POLYMER_TEMPLATE, &insertion_rules(), 10).unwrap();
        let expected = 1588;
        assert_eq!(expected, actual);
    }
    //
    // #[test]
    // fn test_part_2() {}
}
