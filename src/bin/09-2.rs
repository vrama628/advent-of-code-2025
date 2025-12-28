use std::{
    collections::{BTreeMap, BinaryHeap, HashMap},
    ops::RangeInclusive,
    str::FromStr,
};

use itertools::Itertools;
use num_enum::{IntoPrimitive, TryFromPrimitive};

struct Aread<T> {
    area: usize,
    element: T,
}

impl<T> PartialEq for Aread<T> {
    fn eq(&self, other: &Self) -> bool {
        self.area == other.area
    }
}

impl<T> Eq for Aread<T> {}

impl<T> PartialOrd for Aread<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Aread<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.area.cmp(&other.area)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s.split_once(',').unwrap();
        let x = x_str.parse::<usize>().unwrap();
        let y = y_str.parse::<usize>().unwrap();
        Ok(Point { x, y })
    }
}

impl Point {
    fn area_inclusive(&self, other: &Self) -> usize {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }

    fn quadrant(&self, other: &Self) -> Option<Quadrant> {
        if other.x == self.x || other.y == self.y {
            return None;
        }
        let left = (other.x < self.x) as i8;
        let x_and_y_different_direction = (other.x.cmp(&self.x) != other.y.cmp(&self.y)) as i8;
        Some(
            ((left << 1) + x_and_y_different_direction)
                .try_into()
                .unwrap(),
        )
    }
}

#[derive(TryFromPrimitive, IntoPrimitive, PartialEq, Eq, Debug, Clone, Copy)]
#[repr(i8)]
enum Direction {
    PlusY = 0,
    PlusX,
    MinusY,
    MinusX,
}

impl From<(Point, Point)> for Direction {
    fn from((a, b): (Point, Point)) -> Self {
        let decreasing = (b < a) as i8;
        let horizontal = (a.y == b.y) as i8;
        ((decreasing << 1) + horizontal).try_into().unwrap()
    }
}

#[derive(TryFromPrimitive, IntoPrimitive, PartialEq, Eq, Debug, Clone, Copy)]
#[repr(i8)]
enum Turn {
    Clockwise = 1,
    Counterclockwise = -1,
}

impl From<(Direction, Direction)> for Turn {
    fn from((a, b): (Direction, Direction)) -> Self {
        let diff = i8::from(b) - i8::from(a);
        // clamp to range [-1, +1]:
        //   -3 => +1
        //   +3 => -1
        let turn = (diff + 2).rem_euclid(4) - 2;
        turn.try_into().unwrap()
    }
}

impl Turn {
    fn loop_direction(turns: impl Iterator<Item = Self>) -> Self {
        let sum: i8 = turns.map(i8::from).sum();
        if sum == 4 {
            Self::Clockwise
        } else if sum == -4 {
            Self::Counterclockwise
        } else {
            panic!("invalid loop: {sum}")
        }
    }
}

#[derive(PartialEq, Eq, Debug, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(i8)]
enum Quadrant {
    UpperRight = 0,
    LowerRight,
    LowerLeft,
    UpperLeft,
}

impl Quadrant {
    fn inverse(&self) -> Vec<Self> {
        let this = i8::from(*self);
        vec![
            Self::try_from((this + 1) % 4).unwrap(),
            Self::try_from((this + 2) % 4).unwrap(),
            Self::try_from((this + 3) % 4).unwrap(),
        ]
    }

    fn interiors(directions: (Direction, Direction), loop_direction: Turn) -> Vec<Self> {
        let turn = Turn::from(directions);
        let wrapped_quadrant =
            i8::from(directions.1) - (matches!(turn, Turn::Counterclockwise) as i8);
        let wrapped_quadrant = Self::try_from(wrapped_quadrant.rem_euclid(4)).unwrap();
        if turn == loop_direction {
            vec![wrapped_quadrant]
        } else {
            Self::inverse(&wrapped_quadrant)
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Direction, Point, Quadrant, Turn};
    const UPPER_LEFT: Point = Point { x: 0, y: 10 };
    const UPPER_RIGHT: Point = Point { x: 10, y: 10 };
    const LOWER_RIGHT: Point = Point { x: 10, y: 0 };
    const LOWER_LEFT: Point = Point { x: 0, y: 0 };

    #[test]
    fn test_direction_from_points() {
        assert_eq!(Direction::from((UPPER_LEFT, UPPER_RIGHT)), Direction::PlusX);
        assert_eq!(
            Direction::from((UPPER_RIGHT, LOWER_RIGHT)),
            Direction::MinusY
        );
        assert_eq!(
            Direction::from((LOWER_RIGHT, LOWER_LEFT)),
            Direction::MinusX
        );
        assert_eq!(Direction::from((LOWER_LEFT, UPPER_LEFT)), Direction::PlusY);
    }

    #[test]
    fn test_turn() {
        assert_eq!(
            Turn::from((Direction::PlusY, Direction::PlusX)),
            Turn::Clockwise
        );
        assert_eq!(
            Turn::from((Direction::PlusX, Direction::MinusY)),
            Turn::Clockwise
        );
        assert_eq!(
            Turn::from((Direction::MinusY, Direction::MinusX)),
            Turn::Clockwise
        );
        assert_eq!(
            Turn::from((Direction::MinusX, Direction::PlusY)),
            Turn::Clockwise
        );
        assert_eq!(
            Turn::from((Direction::PlusY, Direction::MinusX)),
            Turn::Counterclockwise
        );
        assert_eq!(
            Turn::from((Direction::MinusX, Direction::MinusY)),
            Turn::Counterclockwise
        );
        assert_eq!(
            Turn::from((Direction::MinusY, Direction::PlusX)),
            Turn::Counterclockwise
        );
        assert_eq!(
            Turn::from((Direction::PlusX, Direction::PlusY)),
            Turn::Counterclockwise
        );
    }

    #[test]
    fn test_point_quadrant() {
        assert_eq!(
            UPPER_LEFT.quadrant(&LOWER_RIGHT),
            Some(Quadrant::LowerRight)
        );
        assert_eq!(UPPER_RIGHT.quadrant(&LOWER_LEFT), Some(Quadrant::LowerLeft));
        assert_eq!(LOWER_RIGHT.quadrant(&UPPER_LEFT), Some(Quadrant::UpperLeft));
        assert_eq!(
            LOWER_LEFT.quadrant(&UPPER_RIGHT),
            Some(Quadrant::UpperRight)
        );
        assert_eq!(UPPER_LEFT.quadrant(&UPPER_RIGHT), None);
        assert_eq!(UPPER_LEFT.quadrant(&LOWER_LEFT), None);
    }

    #[test]
    fn test_quadrant_inverse() {
        assert_eq!(
            Quadrant::UpperRight.inverse(),
            vec![
                Quadrant::LowerRight,
                Quadrant::LowerLeft,
                Quadrant::UpperLeft
            ]
        )
    }

    #[test]
    fn test_quadrant_interiors() {
        assert_eq!(
            Quadrant::interiors((Direction::PlusY, Direction::PlusX), Turn::Clockwise),
            vec![Quadrant::LowerRight]
        );
        assert_eq!(
            Quadrant::interiors((Direction::PlusX, Direction::PlusY), Turn::Clockwise),
            vec![
                Quadrant::UpperRight,
                Quadrant::LowerRight,
                Quadrant::LowerLeft
            ]
        );
        assert_eq!(
            Quadrant::interiors((Direction::PlusY, Direction::PlusX), Turn::Counterclockwise),
            vec![
                Quadrant::LowerLeft,
                Quadrant::UpperLeft,
                Quadrant::UpperRight
            ]
        );
        assert_eq!(
            Quadrant::interiors((Direction::PlusX, Direction::PlusY), Turn::Counterclockwise),
            vec![Quadrant::UpperLeft]
        );
    }
}

/// returns (x, y) such that x <= y
fn sort(a: usize, b: usize) -> (usize, usize) {
    (a.min(b), a.max(b))
}

fn main() {
    let points: Vec<Point> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let mut largest_areas: BinaryHeap<Aread<(Point, Point)>> = points
        .iter()
        .copied()
        .cartesian_product(points.clone())
        .filter(|(p1, p2)| p1 < p2)
        .map(|(p1, p2)| Aread {
            area: p1.area_inclusive(&p2),
            element: (p1, p2),
        })
        .collect();

    // for each red tile, store which quadrants around it are green
    let interior_quadrants: HashMap<Point, Vec<Quadrant>> = {
        // when first traveling around the perimeter, we don't know which side is
        // interior until we get to the end to know whether the loop was clockwise or counterclockwise.
        // Start by assuming it's clockwise, then when we get to the end, if it was counterclockwise,
        // negate the collected quadrant information.
        let edges = points
            .iter()
            .copied()
            .circular_tuple_windows::<(Point, Point)>()
            .map(Direction::from);
        // turn at position 0 corresponds to point at position 1
        let mut turns = edges
            .circular_tuple_windows::<(Direction, Direction)>()
            .collect_vec();
        let loop_direction = Turn::loop_direction(turns.iter().copied().map(Turn::from));
        // make it so position 0 corresponds to point 0
        turns.rotate_right(1);
        let interior_quadrants = turns
            .into_iter()
            .map(|turn| Quadrant::interiors(turn, loop_direction));
        points.iter().copied().zip(interior_quadrants).collect()
    };
    // make sure there were no duplicate points clobbering each other
    debug_assert_eq!(interior_quadrants.len(), points.len());

    // store vertical and horizontal lines in a way that makes it easy to check
    // whether any intersect with a candidate rectangle
    let mut vertical_x_to_ys: BTreeMap<usize, Vec<RangeInclusive<usize>>> = BTreeMap::new();
    let mut horizontal_y_to_xs: BTreeMap<usize, Vec<RangeInclusive<usize>>> = BTreeMap::new();
    for (p1, p2) in points.into_iter().circular_tuple_windows() {
        if p1.x == p2.x {
            let (lo_y, hi_y) = sort(p1.y, p2.y);
            vertical_x_to_ys.entry(p1.x).or_default().push(lo_y..=hi_y);
        } else if p1.y == p2.y {
            let (lo_x, hi_x) = sort(p1.x, p2.x);
            horizontal_y_to_xs
                .entry(p1.y)
                .or_default()
                .push(lo_x..=hi_x);
        } else {
            panic!("angled greens {p1:?}->{p2:?}")
        }
    }

    let res = loop {
        let Aread {
            area,
            element: (p1, p2),
        } = largest_areas.pop().unwrap();

        // check that the rectangle is on an interior quadrant of each corner
        let is_interior_at_p1 = p1
            .quadrant(&p2)
            .is_none_or(|quadrant| interior_quadrants[&p1].contains(&quadrant));
        let is_interior_at_p2 = p2
            .quadrant(&p1)
            .is_none_or(|quadrant| interior_quadrants[&p2].contains(&quadrant));
        if !(is_interior_at_p1 && is_interior_at_p2) {
            continue;
        }

        let (x_lo, x_hi) = sort(p1.x, p2.x);
        let (y_lo, y_hi) = sort(p1.y, p2.y);

        // check that no vertical lines intersect the rectangle
        let exists_intersecting_vertical_line = vertical_x_to_ys
            .range(x_lo + 1..x_hi)
            .flat_map(|(_, y_ranges)| y_ranges)
            .any(|y_range| *y_range.start() < y_hi && *y_range.end() > y_lo);
        if exists_intersecting_vertical_line {
            continue;
        }

        // check that no horizontal lines intersect the rectangle
        let exists_intersecting_horizontal_line = horizontal_y_to_xs
            .range(y_lo + 1..y_hi)
            .flat_map(|(_, x_ranges)| x_ranges)
            .any(|x_range| *x_range.start() < x_hi && *x_range.end() > x_lo);
        if exists_intersecting_horizontal_line {
            continue;
        }

        break area;
    };
    println!("{res}");
}
