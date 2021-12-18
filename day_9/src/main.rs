use std::collections::{BinaryHeap, VecDeque};

static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    let heights_map = parse_input(INPUT)?;

    let part_1_result = part_1(&heights_map);
    println!("Part 1 result: {}", part_1_result);

    let part_2_result = part_2(&heights_map);
    println!("Part 2 result: {}", part_2_result);

    Ok(())
}

fn parse_input(input: &str) -> Result<HeightsMap, String> {
    let heights = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(|u| u as usize)
                        .ok_or(format!("{} is not a valid digit", c))
                })
                .collect::<Result<Vec<usize>, String>>()
        })
        .collect::<Result<Vec<Vec<usize>>, String>>()?;
    HeightsMap::new(heights)
}

fn part_1(heights_map: &HeightsMap) -> usize {
    let mut risk_levels_sum: usize = 0;
    for x in 0..=heights_map.x_max {
        for y in 0..=heights_map.y_max {
            for risk_level in heights_map.low_point_risk_level(x, y) {
                risk_levels_sum += risk_level;
            }
        }
    }
    risk_levels_sum
}

fn part_2(heights_map: &HeightsMap) -> usize {
    let mut filled_heights_map = heights_map.heights.clone();
    let mut basin_areas = BinaryHeap::new(); //Max heap
    let mut fill_queue: VecDeque<(usize, usize)> = VecDeque::new();
    for x_start in 0..=heights_map.x_max {
        for y_start in 0..=heights_map.y_max {
            if filled_heights_map[x_start][y_start] < 9 {
                let mut area: usize = 0;
                fill_queue.push_back((x_start, y_start));
                filled_heights_map[x_start][y_start] = 9;

                while let Some((next_x, next_y)) = fill_queue.pop_front() {
                    for (x, y) in heights_map.neighbor_points(next_x, next_y) {
                        if filled_heights_map[x][y] < 9 {
                            fill_queue.push_back((x, y));
                            filled_heights_map[x][y] = 9;
                        }
                    }
                    area += 1;
                }

                basin_areas.push(area);
            }
        }
    }
    basin_areas.iter().take(3).fold(1, |acc, next| acc * next)
}

struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn is_neighbor(&self, other: &Coord) -> bool {
        (self.x == other.x && (self.y == other.y - 1 || self.y == other.y + 1))
            || (self.y == other.y && (self.x == other.x - 1 || self.x == other.x + 1))
    }
}

struct HeightsMap {
    heights: Vec<Vec<usize>>,
    x_max: usize,
    y_max: usize,
}

impl HeightsMap {
    fn new(heights: Vec<Vec<usize>>) -> Result<HeightsMap, String> {
        let x_len = heights.len();
        if x_len == 0 {
            Err("HeightsMap with empty x dimension".to_owned())
        } else {
            let y_len = heights[0].len();
            if y_len == 0 {
                Err("HeightsMap with empty y dimension".to_owned())
            } else if heights.iter().find(|line| line.len() != y_len).is_some() {
                Err("HeightsMap with inconsistent y dimensions".to_owned())
            } else {
                Ok(HeightsMap { heights, x_max: x_len - 1, y_max: y_len - 1 })
            }
        }
    }
    fn height(&self, x: usize, y: usize) -> usize {
        self.heights[x][y]
    }
    fn neighbor_points(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.x_max {
            neighbors.push((x + 1, y))
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < self.y_max {
            neighbors.push((x, y + 1))
        }
        neighbors
    }
    fn neighbor_heights(&self, x: usize, y: usize) -> Vec<usize> {
        self.neighbor_points(x, y)
            .iter()
            .map(|(x, y)| self.height(*x, *y))
            .collect()
    }
    fn low_point_risk_level(&self, x: usize, y: usize) -> Option<usize> {
        let height = self.height(x, y);
        let neighbors = self.neighbor_heights(x, y);
        if neighbors
            .iter()
            .find(|neighbor| **neighbor <= height)
            .is_some()
        {
            None
        } else {
            Some(height + 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_map() -> HeightsMap {
        HeightsMap::new(vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ])
        .unwrap()
    }

    #[test]
    fn test_part_1() {
        let actual = part_1(&test_map());
        let expected: usize = 15;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let actual = part_2(&test_map());
        let expected: usize = 1134;
        assert_eq!(expected, actual);
    }
}
