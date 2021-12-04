use std::str::FromStr;

pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let name = split.next().ok_or("Missing command name")?;
        let value_str = split.next().ok_or("Missing command value")?;
        let value = value_str
            .parse::<i32>()
            .map_err(|_| format!("Invalid command value {}", value_str))?;
        match name {
            "forward" => Ok(Command::Forward(value)),
            "down" => Ok(Command::Down(value)),
            "up" => Ok(Command::Up(value)),
            _ => Err(format!("Invalid command name {}", name)),
        }
    }
}
