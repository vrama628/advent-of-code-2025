use std::str::FromStr;

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

type Distance = usize;

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

const DIAL_SIZE: usize = 100;

impl Rotation {
    fn apply(&self, dial: usize) -> usize {
        let diff = match self.direction {
            Direction::Left => DIAL_SIZE - (self.distance % DIAL_SIZE),
            Direction::Right => self.distance,
        };
        (dial + diff) % DIAL_SIZE
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Rotation parse error: {0}")]
    RotationParse(#[from] RotationParseError),
}

fn main() -> Result<(), Error> {
    let mut dial = 50;
    let mut zeros = 0;
    for line in std::io::stdin().lines() {
        let rotation: Rotation = line?.parse()?;
        dial = rotation.apply(dial);
        if dial == 0 {
            zeros += 1;
        }
    }
    println!("{zeros}");
    Ok(())
}
