use std::collections::HashSet;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash)]
pub enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl TryFrom<char> for Segment {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' => Ok(Segment::A),
            'b' => Ok(Segment::B),
            'c' => Ok(Segment::C),
            'd' => Ok(Segment::D),
            'e' => Ok(Segment::E),
            'f' => Ok(Segment::F),
            'g' => Ok(Segment::G),
            _ => Err(format!("'{}' is not a valid segment value", value)),
        }
    }
}

pub struct SignalPattern(pub HashSet<Segment>);

impl FromStr for SignalPattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments = s
            .chars()
            .map(Segment::try_from)
            .collect::<Result<HashSet<Segment>, String>>()?;
        Ok(SignalPattern(segments))
    }
}
