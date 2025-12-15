use std::{fmt::Display, str::FromStr};

use serde::Deserialize;
use strum::EnumString;

fn main() -> color_eyre::Result<()> {
    let input = std::fs::read_to_string("inputs/day1.txt")?;
    let mut position = Position::new(50);
    let mut pointing_at_zero_count = 0u64;
    let mut zero_clicks_count = 0u64;
    for line in input.lines() {
        let rotation = line.parse::<Rotation>()?;
        zero_clicks_count += position.rotate(&rotation);
        if position.0 == 0 {
            pointing_at_zero_count += 1;
        }
    }
    println!(
        "Number of times pointing at zero: {}",
        pointing_at_zero_count
    );
    println!("Number of zero clicks: {zero_clicks_count}");
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
    degrees: i32,
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
            .parse::<i32>()
            .map_err(|e| eyre::eyre!("Failed to parse degrees: {}", e))?;
        Ok(Rotation { direction, degrees })
    }
}

struct Position(u8);

impl Position {
    pub fn new(position: u8) -> Self {
        Position(position % 100)
    }

    pub fn rotate(&mut self, rotation: &Rotation) -> u64 {
        let delta = match rotation.direction {
            Direction::Left => -rotation.degrees,
            Direction::Right => rotation.degrees,
        };

        let old_pos = self.0 as i32;
        let new_pos = (old_pos + delta).rem_euclid(100);
        self.0 = new_pos as u8;

        let full_rotations = rotation.degrees as u64 / 100;
        let hit_zero = old_pos != 0
            && match rotation.direction {
                Direction::Left => new_pos > old_pos || new_pos == 0,
                Direction::Right => new_pos < old_pos || new_pos == 0,
            };

        full_rotations + hit_zero as u64
    }
}
