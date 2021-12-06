static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    let initial_fish = parse_input(INPUT)?;
    let part_1_end_state = simulate(&initial_fish, 80);
    println!("Part 1 result: {}", part_1_end_state.len());
    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<usize>, String> {
    input
        .lines()
        .next()
        .ok_or("Missing first line")?
        .split(',')
        .map(|fish| fish.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| err.to_string())
}

fn simulate(initial_fish: &[usize], days: usize) -> Vec<usize> {
    let mut all_fish: Vec<usize> = Vec::from(initial_fish);
    for _ in 0..days {
        let mut new_fish = 0;
        for fish in all_fish.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new_fish += 1;
            } else {
                *fish -= 1;
            }
        }
        all_fish.extend(std::iter::repeat(8).take(new_fish));
    }
    all_fish
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_FISH: [usize; 5] = [3, 4, 3, 1, 2];

    #[test]
    fn test() {
        let actual_1 = simulate(&TEST_FISH, 1);
        let expected_1: Vec<usize> = vec![2, 3, 2, 0, 1];
        assert_eq!(expected_1, actual_1);

        let actual_18 = simulate(&TEST_FISH, 18);
        let expected_18: Vec<usize> = vec![
            6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8,
        ];
        assert_eq!(expected_18, actual_18);

        let count_60 = simulate(&TEST_FISH, 80).len();
        assert_eq!(5934, count_60);
    }
}
