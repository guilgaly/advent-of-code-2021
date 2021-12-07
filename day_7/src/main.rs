static INPUT: &str = include_str!("input");

/// Index = crab position, value = number of crabs with that position.
/// E.g. if `crabs[2] == 5`, there are 5 crabs at position 2.
type Crabs = [usize];

fn main() -> Result<(), String> {
    let initial_positions = parse_input(INPUT)?;

    let part_1_result = lowest_fuel_expenditure(&initial_positions, linear_fuel_cost)
        .ok_or("Result 1 not found")?;
    println!("Part 1 result: {}", part_1_result);

    let part_2_result = lowest_fuel_expenditure(&initial_positions, actual_fuel_cost)
        .ok_or("Result 1 not found")?;
    println!("Part 2 result: {}", part_2_result);

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<usize>, String> {
    let crab_positions = input
        .lines()
        .next()
        .ok_or("Missing first line")?
        .split(',')
        .map(|fish| fish.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| err.to_string())?;
    let crabs = crab_positions_to_distribution(&crab_positions)?;
    Ok(crabs)
}

fn crab_positions_to_distribution(crab_positions: &[usize]) -> Result<Vec<usize>, String> {
    let max_position = crab_positions.iter().max().ok_or("Empty input list")?;
    let mut all_crabs: Vec<usize> = vec![0; max_position + 1];
    for crab_position in crab_positions {
        all_crabs[*crab_position] += 1;
    }
    Ok(all_crabs)
}

fn lowest_fuel_expenditure(
    initial_positions: &Crabs,
    fuel_cost: fn(usize) -> usize,
) -> Option<usize> {
    let calculate_fuel_expenditure = |target: usize| -> usize {
        initial_positions
            .iter()
            .enumerate()
            .fold(0, |acc, (pos, count)| {
                acc + count * fuel_cost(abs_diff(pos, target))
            })
    };

    (0..initial_positions.len())
        .into_iter()
        .map(calculate_fuel_expenditure)
        .min()
}

fn linear_fuel_cost(distance: usize) -> usize {
    distance
}

fn actual_fuel_cost(distance: usize) -> usize {
    // https://en.wikipedia.org/wiki/1_%2B_2_%2B_3_%2B_4_%2B_%E2%8B%AF#Partial_sums
    distance * (distance + 1) / 2
}

fn abs_diff(x: usize, y: usize) -> usize {
    if x < y {
        y - x
    } else {
        x - y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_CRABS: [usize; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_part_1() {
        let crabs = crab_positions_to_distribution(&TEST_CRABS).unwrap();
        let actual = lowest_fuel_expenditure(&crabs, linear_fuel_cost).unwrap();
        let expected: usize = 37;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let crabs = crab_positions_to_distribution(&TEST_CRABS).unwrap();
        let actual = lowest_fuel_expenditure(&crabs, actual_fuel_cost).unwrap();
        let expected: usize = 168;
        assert_eq!(expected, actual);
    }
}
