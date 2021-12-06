static INPUT: &str = include_str!("input");

const NEW_FISH_TIMER: usize = 8;
const RESET_FISH_TIMER: usize = 6;
const MAX_FISH_TIMER: usize = NEW_FISH_TIMER;

type Fish = [usize; MAX_FISH_TIMER + 1];

fn main() -> Result<(), String> {
    let initial_fish = parse_input(INPUT)?;

    let part_1_end_state = simulate(&initial_fish, 80);
    println!("Part 1 result: {}", count_fish(&part_1_end_state));

    let part_2_end_state = simulate(&initial_fish, 256);
    println!("Part 2 result: {}", count_fish(&part_2_end_state));

    Ok(())
}

fn parse_input(input: &str) -> Result<Fish, String> {
    let fish_vec = input
        .lines()
        .next()
        .ok_or("Missing first line")?
        .split(',')
        .map(|fish| fish.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| err.to_string())?;

    let max_value = fish_vec.iter().max().ok_or("Empty initial fish")?;
    if *max_value > MAX_FISH_TIMER {
        return Err(format!("Illegal fish value {}", max_value));
    }

    let mut all_fish: Fish = [0; MAX_FISH_TIMER + 1];
    for fish in fish_vec {
        all_fish[fish] += 1;
    }
    Ok(all_fish)
}

fn simulate(initial_fish: &Fish, days: usize) -> Fish {
    let mut all_fish = initial_fish.clone();

    for _ in 0..days {
        let birth_rate = all_fish[0];
        for timer in 0..MAX_FISH_TIMER {
            all_fish[timer] = all_fish[timer + 1];
        }
        all_fish[RESET_FISH_TIMER] += birth_rate;
        all_fish[NEW_FISH_TIMER] = birth_rate;
    }

    all_fish
}

fn count_fish(fish: &Fish) -> usize {
    fish.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_FISH: Fish = [0, 1, 1, 2, 1, 0, 0, 0, 0];

    #[test]
    fn test() {
        let actual_1 = simulate(&TEST_FISH, 1);
        let expected_1: Fish = [1, 1, 2, 1, 0, 0, 0, 0, 0];
        assert_eq!(expected_1, actual_1);

        let actual_18 = simulate(&TEST_FISH, 18);
        let expected_18: Fish = [3, 5, 3, 2, 2, 1, 5, 1, 4];
        assert_eq!(expected_18, actual_18);

        let actual_80 = simulate(&TEST_FISH, 80);
        assert_eq!(5934, count_fish(&actual_80));

        let actual_256 = simulate(&TEST_FISH, 256);
        assert_eq!(26984457539, count_fish(&actual_256));
    }
}
