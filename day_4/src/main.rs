use common::itertools::Itertools;
use std::convert::TryInto;

static INPUT: &str = include_str!("input");

struct Game {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

struct Board {
    rows: [[usize; 5]; 5],
}

impl Board {
    fn is_complete(&self, called_numbers: &[usize]) -> bool {
        let is_row_complete = |y: usize| -> bool {
            for x in 0..5 {
                if !called_numbers.contains(&self.rows[y][x]) {
                    return false;
                }
            }
            true
        };
        for y in 0..5 {
            if is_row_complete(y) {
                return true;
            }
        }

        let is_column_complete = |x: usize| -> bool {
            for y in 0..5 {
                if !called_numbers.contains(&self.rows[y][x]) {
                    return false;
                }
            }
            true
        };
        for x in 0..5 {
            if is_column_complete(x) {
                return true;
            }
        }

        false
    }

    fn score(&self, called_numbers: &[usize]) -> Option<usize> {
        called_numbers.last().map(|last_called| {
            let sum: usize = self
                .rows
                .iter()
                .map(|row| row.iter())
                .flatten()
                .filter(|number| !called_numbers.contains(number))
                .sum();
            sum * last_called
        })
    }
}

fn main() -> Result<(), String> {
    let game = parse_input(INPUT)?;
    {
        let (winning_board, called_numbers) =
            find_first_winning_board(&game).ok_or("No winning board found")?;
        let score = winning_board
            .score(&called_numbers)
            .ok_or("Cannot calculate score")?;
        println!("Part 1 result: {}", score);
    }
    {
        let (winning_board, called_numbers) =
            find_last_winning_board(&game).ok_or("No winning board found")?;
        let score = winning_board
            .score(&called_numbers)
            .ok_or("Cannot calculate score")?;
        println!("Part 2 result: {}", score);
    }
    Ok(())
}

fn find_first_winning_board(game: &Game) -> Option<(&Board, Vec<usize>)> {
    for game_length in 1..=game.numbers.len() {
        let numbers: Vec<usize> = game.numbers.iter().take(game_length).copied().collect();
        for board in &game.boards {
            if board.is_complete(&numbers) {
                return Some((board, numbers));
            }
        }
    }
    None
}

fn find_last_winning_board(game: &Game) -> Option<(&Board, Vec<usize>)> {
    let mut numbers = game.numbers.iter();
    let mut called_numbers: Vec<usize> = vec![];
    let mut remaining_boards: Vec<&Board> = game.boards.iter().collect();
    let mut last_winning_board: Option<&Board> = None;
    while let Some(next_number) = numbers.next() {
        called_numbers.push(*next_number);
        remaining_boards.retain(|board| {
            if board.is_complete(&called_numbers) {
                last_winning_board = Some(board);
                false
            } else {
                true
            }
        });
        if remaining_boards.is_empty() {
            break;
        }
    }
    last_winning_board.map(|board| (board, called_numbers))
}

fn parse_input(input: &str) -> Result<Game, String> {
    fn parse_number(number: &str) -> Result<usize, String> {
        number.parse::<usize>().map_err(|err| err.to_string())
    }

    let mut lines = input.lines();

    let numbers = lines
        .next()
        .ok_or("Missing numbers line")?
        .split(',')
        .map(parse_number)
        .collect::<Result<Vec<usize>, String>>()?;

    let boards_lines = lines.chunks(6);
    let boards = (&boards_lines)
        .into_iter()
        .map(|lines| {
            let rows: [[usize; 5]; 5] = lines
                .skip(1)
                .map(|line| {
                    let row: [usize; 5] = line
                        .split_whitespace()
                        .map(parse_number)
                        .collect::<Result<Vec<_>, String>>()?
                        .try_into()
                        .map_err(|vec| format!("{:?} is not of size 5", vec))?;
                    Ok(row)
                })
                .collect::<Result<Vec<_>, String>>()?
                .try_into()
                .map_err(|vec| format!("{:?} is not of size 5", vec))?;
            Ok(Board { rows })
        })
        .collect::<Result<Vec<_>, String>>()?;

    Ok(Game { numbers, boards })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
