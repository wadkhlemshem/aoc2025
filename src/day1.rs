use std::{fmt::Display, str::FromStr};

use serde::Deserialize;
use strum::EnumString;

fn main() -> color_eyre::Result<()> {
    let input = std::fs::read_to_string("inputs/day1.txt")?;
    let mut position = Position::new(50);
    let mut pointing_at_zero_count = 0u64;
    for line in input.lines() {
        let rotation = line.parse::<Rotation>()?;
        position.move_in_direction(&rotation);
        if position.0 == 0 {
            pointing_at_zero_count += 1;
        }
    }
    println!(
        "Number of times pointing at zero: {}",
        pointing_at_zero_count
    );
    Ok(())
}

#[derive(Debug, Deserialize, EnumString, strum::Display)]
enum Direction {
    #[strum(serialize = "L")]
    Left,
    #[strum(serialize = "R")]
    Right,
}

impl TryFrom<char> for Direction {
    type Error = eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(eyre::eyre!("Invalid direction character: {}", value)),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Rotation {
    direction: Direction,
    degrees: u16,
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.direction, self.degrees)
    }
}

impl FromStr for Rotation {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s
            .chars()
            .next()
            .ok_or_else(|| eyre::eyre!("Empty string"))?
            .try_into()?;
        let degrees = s[1..]
            .parse::<u16>()
            .map_err(|e| eyre::eyre!("Failed to parse degrees: {}", e))?;
        Ok(Rotation { direction, degrees })
    }
}

struct Position(u8);

impl Position {
    pub fn new(position: u8) -> Self {
        Position(position % 100)
    }

    pub fn move_in_direction(&mut self, rotation: &Rotation) {
        let degrees = (rotation.degrees % 100) as u8;
        match rotation.direction {
            Direction::Left => {
                self.0 = if self.0 >= degrees {
                    self.0 - degrees
                } else {
                    100 - degrees + self.0
                };
            }
            Direction::Right => {
                self.0 = (self.0 + degrees) % 100;
            }
        }
    }
}
