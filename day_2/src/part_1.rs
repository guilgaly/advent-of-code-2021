use crate::command::Command;

pub fn part_1_result(commands: &[Command]) -> i32 {
    let final_position = follow_commands(commands);
    final_position.horizontal * final_position.depth
}

fn follow_commands(commands: &[Command]) -> Position {
    let initial_pos = Position { horizontal: 0, depth: 0 };
    commands
        .iter()
        .fold(initial_pos, |acc, next| acc.apply_command(next))
}

#[derive(Debug, PartialEq)]
struct Position {
    horizontal: i32,
    depth: i32,
}

impl Position {
    fn apply_command(&self, command: &Command) -> Position {
        match command {
            Command::Forward(x) => Position { horizontal: self.horizontal + x, ..*self },
            Command::Down(x) => Position { depth: self.depth + x, ..*self },
            Command::Up(x) => Position { depth: self.depth - x, ..*self },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: [Command; 6] = [
        Command::Forward(5),
        Command::Down(5),
        Command::Forward(8),
        Command::Up(3),
        Command::Down(8),
        Command::Forward(2),
    ];

    #[test]
    fn test_part_1() -> Result<(), String> {
        let actual = follow_commands(&TEST_INPUT);
        let expected = Position { horizontal: 15, depth: 10 };
        assert_eq!(expected, actual);
        Ok(())
    }
}
