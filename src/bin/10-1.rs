use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use bitvec::array::BitArray;

struct Machine {
    goal: BitArray,
    buttons: Vec<BitArray>,
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once("] (").ok_or(())?;
        let (second, _third) = second.split_once(") {").ok_or(())?;

        let goal_str = first.strip_prefix("[").ok_or(())?;
        let mut goal = BitArray::new([0]);
        for (i, c) in goal_str.char_indices() {
            goal.set(i, c == '#')
        }

        let buttons = second
            .split(") (")
            .map(|schematic| {
                let mut button = BitArray::new([0]);
                for wiring in schematic.split(',') {
                    let i = wiring.parse::<usize>().unwrap();
                    button.set(i, true);
                }
                button
            })
            .collect();

        Ok(Self { goal, buttons })
    }
}

impl Machine {
    /// returns the shortest sequence of button presses to reach this machine's goal
    fn solve(&self) -> usize {
        let start: BitArray = BitArray::new([0]);
        debug_assert_ne!(self.goal, start);
        let mut seen = HashSet::from([start]);
        let mut queue = VecDeque::from([(start, 0)]);
        loop {
            let (node, presses) = queue.pop_front().unwrap();
            let presses = presses + 1;
            for edge in &self.buttons {
                let neighbor = node ^ edge;
                if seen.insert(neighbor) {
                    if neighbor == self.goal {
                        return presses;
                    } else {
                        queue.push_back((neighbor, presses))
                    }
                }
            }
        }
    }
}

fn main() {
    let res = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().parse::<Machine>().unwrap())
        .map(|machine| machine.solve())
        .sum::<usize>();
    println!("{res}")
}
