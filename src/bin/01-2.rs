use std::{fmt::Display, ops::RangeInclusive, str::FromStr};

enum Direction {
    Left,
    Right,
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
struct DirectionParseError(String);

impl FromStr for Direction {
    type Err = DirectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(DirectionParseError(format!(
                "Expected L or R, but received {s}"
            ))),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "L"),
            Direction::Right => write!(f, "R"),
        }
    }
}

type Distance = isize;

struct Rotation {
    direction: Direction,
    distance: Distance,
}

#[derive(Debug, thiserror::Error)]
enum RotationParseError {
    #[error("Error parsing direction: {0}")]
    Direction(#[from] DirectionParseError),
    #[error("Error parsing distance: {0}")]
    Distance(#[from] std::num::ParseIntError),
}

impl FromStr for Rotation {
    type Err = RotationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction_str, distance_str) = s.split_at(1);
        let direction = direction_str.parse()?;
        let distance = distance_str.parse()?;
        Ok(Self {
            direction,
            distance,
        })
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.direction, self.distance)
    }
}

const DIAL_SIZE: isize = 100;

impl Rotation {
    /// does NOT modulo 100
    fn apply(&self, dial: isize) -> isize {
        let diff = match self.direction {
            Direction::Left => -self.distance,
            Direction::Right => self.distance,
        };
        dial + diff
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Rotation parse error: {0}")]
    RotationParse(#[from] RotationParseError),
}

/// inclusive on both ends of the interval
fn zeros_in_interval(interval: RangeInclusive<isize>) -> usize {
    let lower_class = (interval.start() - 1).div_euclid(DIAL_SIZE);
    let upper_class = interval.end().div_euclid(DIAL_SIZE);
    lower_class.abs_diff(upper_class)
}

fn main() -> Result<(), Error> {
    let mut dial = 50;
    // total of numbers of zeros contained in each interval
    let mut zeros = 0;
    for line in std::io::stdin().lines() {
        let rotation: Rotation = line?.parse()?;
        if dial % 100 == 0 {
            // this next interval would double-count the zero at the end of the last rotation
            zeros -= 1;
        }
        let new_dial = rotation.apply(dial);
        let range = dial.min(new_dial)..=dial.max(new_dial);
        zeros += zeros_in_interval(range);
        dial = new_dial;
    }
    println!("{zeros}");
    Ok(())
}
