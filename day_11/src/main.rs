use std::convert::TryInto;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    let octopuses = parse_input(INPUT)?;

    let part_1_result = part_1(&octopuses);
    println!("Part 1 result: {}", part_1_result);

    let part_2_result = part_2(&octopuses);
    println!("Part 2 result: {}", part_2_result);

    Ok(())
}

fn parse_input(input: &str) -> Result<[[u8; 10]; 10], String> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(|u| u as u8)
                        .ok_or(format!("{} is not a valid digit", c))
                })
                .collect::<Result<Vec<u8>, String>>()?
                .try_into()
                .map_err(|_| format!("line {} does not have 10 values", line))
        })
        .collect::<Result<Vec<[u8; 10]>, String>>()?
        .try_into()
        .map_err(|_| "Number of lines is not 10".to_owned())
}

fn part_1(init_octopuses: &[[u8; 10]; 10]) -> usize {
    let mut flashes = 0;
    let mut octopuses = init_octopuses.clone();
    for _ in 0..100 {
        next_step(&mut octopuses);
        flashes += count_flashes(&octopuses);
    }
    flashes
}

fn part_2(init_octopuses: &[[u8; 10]; 10]) -> usize {
    let mut step_count = 0;
    let mut octopuses = init_octopuses.clone();
    while count_flashes(&octopuses) != 100 {
        next_step(&mut octopuses);
        step_count += 1;
    }
    step_count
}

fn next_step(octopuses: &mut [[u8; 10]; 10]) {
    for x in 0..10 {
        for y in 0..10 {
            octopuses[x][y] += 1;
        }
    }
    while handle_flashes(octopuses) {}
}

fn handle_flashes(octopuses: &mut [[u8; 10]; 10]) -> bool {
    let mut new_flash = false;
    for x in 0..10 {
        for y in 0..10 {
            if octopuses[x][y] > 9 {
                octopuses[x][y] = 0;
                new_flash = true;
                increment_neighbors(octopuses, x, y);
            }
        }
    }
    new_flash
}

fn increment_neighbors(octopuses: &mut [[u8; 10]; 10], curr_x: usize, curr_y: usize) {
    let min_x = if curr_x == 0 { 0 } else { curr_x - 1 };
    let max_x = if curr_x == 9 { 9 } else { curr_x + 1 };
    let min_y = if curr_y == 0 { 0 } else { curr_y - 1 };
    let max_y = if curr_y == 9 { 9 } else { curr_y + 1 };
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if octopuses[x][y] != 0 {
                octopuses[x][y] += 1;
            }
        }
    }
}

fn count_flashes(octopuses: &[[u8; 10]; 10]) -> usize {
    octopuses
        .iter()
        .flat_map(|l| l.iter())
        .filter(|octopus| **octopus == 0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: [[u8; 10]; 10] = [
        [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
    ];

    #[test]
    fn test_part_1() {
        let actual = part_1(&TEST_INPUT);
        let expected = 1656;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let actual = part_2(&TEST_INPUT);
        let expected = 195;
        assert_eq!(expected, actual);
    }
}
